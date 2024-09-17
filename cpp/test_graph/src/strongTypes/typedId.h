
template<typename T, typename TId>
class TypedId {
private:
    const TId _id;

public:
    explicit TypedId(TId id) : _id(id) {}
    TId get() const { return _id; } 

    const bool operator==(const TypedId<T, TId>& rhs) const {
        return _id == rhs._id;
    } 

    const bool operator!=(const TypedId<T, TId>& rhs) const {
        return !(this == rhs);
    } 
};

using VectorId = TypedId<int, struct VectorIdTag>;
