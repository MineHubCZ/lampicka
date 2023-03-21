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
            color: "#" + tokens[1],
            speed: tokens[0],
        };
    }

    if (hasColor(mode))  {
        return {
            mode,
            brightness,
            color: "#" + data,
            speed: 125,
        };
    }

    if (hasSpeed(mode)) {
        return {
            mode,
            brightness,
            color: "#ffffff",
            speed: data,
        }
    }
}

export function viceVersa(id: number, top: Mode, bottom: Mode): string {
    return [id, exportMode(top), exportMode(bottom)].join(";");
}

export function exportMode(mode: Mode): string {
    let data;
    if (hasSpeed(mode.mode) && hasColor(mode.mode)) { 
        data = mode.speed + "," + mode.color.replace("#", "");
    } else {
        data = hasSpeed(mode.mode) ? mode.speed : mode.color.replace("#", "");
    }

    return [mode.mode, mode.brightness, data].join(";");
}
