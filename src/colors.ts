export function rainbow(red, green, blue) {
    if (red == 255) {
        if (blue == 0 && green < 255) {
            return [red, ++green, blue];   
        }
        
        if (blue > 0) {
            return [red, green, --blue];
        }
    }
 
    if (blue == 255) {
        if (green == 0) {
            return [++red, green, blue];
        }
        return [red, --green, blue];
    }

    if (green == 255) {
        if (red == 0) {
            return [red, green, ++blue];
        }
        return [--red, green, blue];
    }
}
