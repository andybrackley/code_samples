import * as THREE from 'three';

export class Player {
    constructor() {
        // Physics properties
        this.velocity = new THREE.Vector3();
        this.direction = new THREE.Vector3(0, 0, -1);
        this.thrust = 0.001;
        this.brake = 0.0015;
        this.drag = 0.99;
        this.maxSpeed = 0.5;
        this.rotationSpeed = 0.03;

        // Trail properties
        this.trails = [];
        this.isTrailActive = false;
        this.trailStartTime = 0;
        this.trailFadeDuration = 5000; // 5 seconds in milliseconds

        // Create the 3D model
        this.model = this.createTronBike();
    }

    createTronBike() {
        const bikeGroup = new THREE.Group();
    
        // Main body (curved shape using ExtrudeGeometry)
        const bodyShape = new THREE.Shape();
        bodyShape.moveTo(-1.0, -0.3); // Start at the back
        bodyShape.lineTo(1.0, -0.3);  // Go to the front
        bodyShape.lineTo(1.2, -0.1);  // Front nose up
        bodyShape.lineTo(1.0, 0.3);   // Top front
        bodyShape.lineTo(-1.0, 0.3);  // Top back
        bodyShape.lineTo(-1.0, -0.3); // Close shape
    
        const extrudeSettings = {
            depth: 0.4,
            bevelEnabled: true,
            bevelSegments: 2,
            steps: 2,
            bevelSize: 0.05,
            bevelThickness: 0.05
        };

        const wheelOffset = 0.2;
    
        const bodyGeometry = new THREE.ExtrudeGeometry(bodyShape, extrudeSettings);
        bodyGeometry.rotateY(Math.PI / 2);
        
        const bodyMaterial = new THREE.MeshPhongMaterial({
            color: 0x1a2a44,
            emissive: 0x00ffff,
            specular: 0x00ffff,
            shininess: 50,
            transparent: true,
            opacity: 0.9
        });
    
        const body = new THREE.Mesh(bodyGeometry, bodyMaterial);
        body.position.y = 0.3;
        bikeGroup.add(body);
    
        // Wheels
        const wheelRadius = 0.4;
        const wheelWidth = 0.4;
        const wheelGeometry = new THREE.CylinderGeometry(wheelRadius, wheelRadius, wheelWidth, 32);
        const wheelMaterial = new THREE.MeshPhongMaterial({
            color: 0x00ffff,
            emissive: 0x00cccc,
            transparent: true,
            opacity: 0.8
        });
    
        const frontWheel = new THREE.Mesh(wheelGeometry, wheelMaterial);
        frontWheel.rotation.z = Math.PI / 2;
        frontWheel.position.set(wheelOffset, 0, 1.0);
        bikeGroup.add(frontWheel);
    
        const backWheel = frontWheel.clone();
        backWheel.position.set(wheelOffset, 0, -1.0);
        bikeGroup.add(backWheel);
    
        // Add glowing edges
        const edgesGeometry = new THREE.EdgesGeometry(bodyGeometry);
        const edgesMaterial = new THREE.LineBasicMaterial({ 
            color: 0x00ffff, 
            linewidth: 2
        });
        const edges = new THREE.LineSegments(edgesGeometry, edgesMaterial);
        body.add(edges);
    
        // Point light for extra glow
        const glowLight = new THREE.PointLight(0x00ffff, 1, 5);
        glowLight.position.set(0, 0.5, 0);
        bikeGroup.add(glowLight);
    
        return bikeGroup;
    }

    createTrail() {
        const trailWidth = 0.1;
        const trailHeight = 0.8;
        const trailLength = 0.5;
        
        const trailGeometry = new THREE.BoxGeometry(trailWidth, trailHeight, trailLength);
        trailGeometry.translate(0.2, 0, -0.8);

        const trailMaterial = new THREE.MeshPhongMaterial({
            color: 0x00ffff,
            emissive: 0x00ffff,
            emissiveIntensity: 0.8,
            transparent: true,
            opacity: 0.6,
            side: THREE.DoubleSide
        });
    
        const trail = new THREE.Mesh(trailGeometry, trailMaterial);
        
        // Position relative to the bike's current position
        const bikePos = this.model.position;
        const bikeRot = this.model.rotation;
        
        // Calculate trail position based on bike's orientation
        const direction = new THREE.Vector3(0, 0, -1);
        direction.applyQuaternion(this.model.quaternion);
        
        trail.position.copy(bikePos);
        trail.position.y = (trailHeight/2);
        trail.rotation.copy(bikeRot);
        
        // Move trail behind bike
        trail.position.add(direction.multiplyScalar(trailLength/2));
        
        this.model.parent.add(trail);
        
        return {
            mesh: trail,
            startTime: Date.now(),
            material: trailMaterial
        };
    }

    toggleTrail() {
        this.isTrailActive = !this.isTrailActive;
        if (this.isTrailActive) {
            const trail = this.createTrail();
            this.trails.push(trail);
        }
    }

    updateTrails() {
        const currentTime = Date.now();
        
        // Update existing trails
        for (let i = this.trails.length - 1; i >= 0; i--) {
            const trail = this.trails[i];
            const age = currentTime - trail.startTime;
            
            if (age >= this.trailFadeDuration) {
                // Remove fully faded trail
                this.model.parent.remove(trail.mesh);
                this.trails.splice(i, 1);
            } else if (!this.isTrailActive && age >= 0) {
                // Fade out trail when deactivated
                const fadeProgress = age / this.trailFadeDuration;
                trail.material.opacity = 0.6 * (1 - fadeProgress);
            }
        }
        
        // Create new trail segment if active
        if (this.isTrailActive) {
            const trail = this.createTrail();
            this.trails.push(trail);
        }
    }

    update(keys) {
        // Handle rotation
        if (keys.ArrowLeft) {
            this.model.rotation.y += this.rotationSpeed;
            this.isTrailActive = false;
        }
        if (keys.ArrowRight) {
            this.model.rotation.y -= this.rotationSpeed;
            this.isTrailActive = false;
        }

        if (keys.Space) {
            this.toggleTrail();
        }

        // Get the bike's forward direction
        const forward = new THREE.Vector3(0, 0, 1);
        forward.applyQuaternion(this.model.quaternion);

        // Apply thrust or brake
        if (keys.ArrowUp) {
            const acceleration = forward.multiplyScalar(this.thrust);
            this.velocity.add(acceleration);
        } else if (keys.ArrowDown) {
            this.velocity.multiplyScalar(1 - this.brake);
        }

        // Apply drag
        this.velocity.multiplyScalar(this.drag);

        // Limit speed
        if (this.velocity.length() > this.maxSpeed) {
            this.velocity.normalize().multiplyScalar(this.maxSpeed);
        }

        // Update position
        this.model.position.add(this.velocity);

        // Update trails
        this.updateTrails();
    }

    getPosition() {
        return this.model.position;
    }

    getModel() {
        return this.model;
    }
}