fn main() {
    {
        // Send를 사용하여 스레드 사이에 소유권 이동을 허용하기
        // Send가 구현된 타입의 소유권이 스레드 사이에서 이동될 수 있음
        // Rc<T>는 여러분이 스레드-안전성 성능 저하를 지불하지 않아도 되는 싱글스레드의 경우에 사용되도록 구현
        // Rust의 타입 시스템은 스레드 간에 Rc<T> 값을 불안전하게 보내질 수 없도록 보장
    }
    {
        // Sync를 사용하여 여러 스레드로부터의 접근을 허용하기
        // Rc<T>는 또한 Send가 아닌 이유와 동일한 이유로 Sync하지도 않음
        // RefCell<T> 타입과 연관된 Cell<T> 타입도 Sync 하지못함.
        // 스마트 포인터 Mutex<T>는 Sync 하며, 여러 스데르에서 접근을 공유하는데 사용 될 수 있음.
    }

    {
        // 메시지 패싱을 위한 채널을 제공하고,
        // 동시적 컨텍스트에서 사용하기 안전한 Mutex<T>와 Arc<T> 같은 스마트 포인터 타입들을 제공
    }
}