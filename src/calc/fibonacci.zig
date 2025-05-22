pub fn v1_recursive(n: usize) usize {
    if (n == 0) {
        return n;
    }
    if (n == 1) {
        return n;
    }
    return v1_recursive(n - 1) + v1_recursive(n - 2);
}

const std = @import("std");
