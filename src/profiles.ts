export interface Profile {
    id: number;
    mode1: Mode;
    mode2: Mode;
}

export interface Mode {
    mode: string;
    brightness: number;
    color?: string;
    speed?: number;
}

export function hasSpeed(mode) {
    return mode != "s";
}

export function hasColor(mode) {
    return ["s", "w", "b"].includes(mode);
}

export function parse(profile: string): Profile {
    let [id, mode1, brightness1, data1, mode2, brightness2, data2] = profile.split(";");
    return {
        id,
        mode1: parseMode(mode1, brightness1, data1),
        mode2: parseMode(mode2, brightness2, data2),
    }
}

function parseMode(mode: string, brightness: number, data: string): Mode {
    if (hasColor(mode) && hasSpeed(mode)) {
        let tokens = data.split(',');
        return {
            mode,
            brightness,
            color: tokens[1],
            speed: tokens[0],
        };
    }

    if (hasColor(mode))  {
        return {
            mode,
            brightness,
            color: data,
        };
    }

    if (hasSpeed(mode)) {
        return {
            mode,
            brightness,
            speed: data,
        }
    }
}

export function viceVersa(id: number, top: Mode, bottom: Mode): string {
    return [id, exportMode(top), exportMode(bottom)].join(";");
}

export function exportMode(mode: Mode): string {
    let data;
    if (hasSpeed(mode) && hasColor(mode)) {
        data = mode.speed + "," + mode.color;
    } else {
        data = mode.speed ?? mode.color;
    }

    return [mode.mode, mode.brightness, data].join(";");
}
