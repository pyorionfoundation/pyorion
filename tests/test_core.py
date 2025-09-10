def test_add() -> int:
    # simple dummy function, replace later with your Rust extension
    def add(a: int, b: int):
        return a + b

    assert add(2, 3) == 5
    assert add(-1, 1) == 0
    assert add(0, 0) == 0
