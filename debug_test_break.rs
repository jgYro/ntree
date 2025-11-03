fn test_break() {
    let mut x = 0;
    while x < 100 {
        if x > 5 {
            break;
        }
        x = x + 1;
    }
    return x;
}