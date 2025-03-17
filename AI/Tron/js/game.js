import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { Player } from './player.js';

export class Game {
    constructor() {
        this.scene = new THREE.Scene();
        this.camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
        this.renderer = new THREE.WebGLRenderer({
            canvas: document.querySelector('#gameCanvas'),
            antialias: true
        });

        // Camera settings
        this.cameraHeight = 2;
        this.cameraDistance = 10;

        // Input state
        this.keys = {
            ArrowUp: false,
            ArrowDown: false,
            ArrowLeft: false,
            ArrowRight: false,
            Space: false
        };
    }

    init() {
        // Setup renderer
        this.renderer.setSize(window.innerWidth, window.innerHeight);
        this.renderer.setPixelRatio(window.devicePixelRatio);

        // Setup camera
        this.camera.position.set(0, this.cameraHeight, this.cameraDistance);
        this.camera.lookAt(0, 0, 0);

        // Add orbit controls
        this.controls = new OrbitControls(this.camera, this.renderer.domElement);
        this.controls.enableDamping = true;

        // Create and add player
        this.player = new Player(this.scene);
        this.scene.add(this.player.getModel());

        // Add lights
        const light = new THREE.DirectionalLight(0xffffff, 1);
        light.position.set(1, 1, 1);
        this.scene.add(light);
        this.scene.add(new THREE.AmbientLight(0x404040));

        // Add a grid for reference
        const gridHelper = new THREE.GridHelper(200, 200);
        this.scene.add(gridHelper);

        // Setup input handlers
        window.addEventListener('keydown', (e) => this.handleKeyDown(e));
        window.addEventListener('keyup', (e) => this.handleKeyUp(e));
        window.addEventListener('resize', () => this.onWindowResize(), false);

        // Start animation loop
        this.animate();
    }

    toggleKey(event, isKeydown) {
        const key = event.key === ' ' ? 'Space' : event.key;
        if (key in this.keys) {
            this.keys[key] = isKeydown;
        }
    }

    handleKeyDown(event) {
        this.toggleKey(event, true);
    }

    handleKeyUp(event) {
        this.toggleKey(event, false);
    }

    onWindowResize() {
        this.camera.aspect = window.innerWidth / window.innerHeight;
        this.camera.updateProjectionMatrix();
        this.renderer.setSize(window.innerWidth, window.innerHeight);
    }

    animate() {
        requestAnimationFrame(() => this.animate());
        
        // Update player
        this.player.update(this.keys);
        
        // Make camera follow the player from behind
        const playerPos = this.player.getPosition();
        this.camera.position.x = playerPos.x;
        this.camera.position.y = playerPos.y + this.cameraHeight;
        this.camera.position.z = playerPos.z + this.cameraDistance;
        this.controls.target.copy(playerPos);
        
        this.controls.update();
        this.renderer.render(this.scene, this.camera);
    }
}
