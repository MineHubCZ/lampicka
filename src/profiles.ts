export interface Profile {
    id: number;
    mode1: string;
    brightness1: number;
    data1: string[];
    mode2?: string;
    brightness2?: number;
    data2?: string[];
}

export function parse(profile: string): Profile {
    let parts = profile.split(";");
    let base = {
            id: +parts[0],
            mode1: parts[1],
            brightness1: +parts[2],
            data1: parts[3].split(','),
            mode2: null,
            brightness2: null,
            data2: null,
    };
    
    if (parts[1] == "a") {
        return base;
    }

    base.mode2 = parts[4];
    base.brightness2 = parts[5];
    base.data2 = parts[6].split(',');

    return base;
}
