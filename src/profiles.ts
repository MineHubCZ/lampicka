export type Profile = {
    id: number;
    opacity1: number;
    opacity2?: number;
    modes: [Mode, Mode] | IndividualGradAll; 
}

export function exportProfile(profile: Profile): string {
    if (profile.modes instanceof IndividualGradAll) {
        return profile.id.toString() + profile.modes.export().join(";" + profile.opacity1.toString() + ";");
    }

    let first = profile.modes[0].export().join(";" + profile.opacity1.toString() + ";");
    let second = profile.modes[1].export().join(";" + profile.opacity2.toString() + ";");
    
    return profile.id.toString() + ";" + first + ";" + second;
}

export type Mode = Static | Rainbow | RainbowGrad | Individual | IndividualGrad;

export type Static = {
    color: string;
}

export function export(mode: Static): [string, string] {
    return ["s", mode.color];
}

export type Rainbow = {
    speed: number;
}

export function export(mode: Rainbow): [string, string] {
    return ["r", rainbow.speed.toString()]
}

export type RainbowGrad = { 
    speed: number;
    export(): [string, string] {
        return ["rg", this.speed.toString()];
    }
}

export function export(mode: RainbowGrad): [string, string] {
    return ["r", rainbow.speed.toString()]
}

export type Individual = {
    colors: [string, string, string, string, string] | [string, string, string];   
}

export function export(mode: Individual): [string, string] {
    return ["i", this.colors.join(",")];
}

export type IndividualGrad = {
    colors: [string, string, string, string, string] | [string, string, string];   
    speed: number;
}

export function export(mode: IndividualGrad): [string, string] {
    return ["ig", this.speed.toString() + "," + this.colors.join(",")];
}

export type IndividualGradAll = {
    colors: [string, string, string, string, string, string, string, string];
    speed: number; 
}

export function export(mode: IndividualGrad): [string, string] {
    return ["a", this.speed.toString() + "," + this.colors.join(",")];
}

export function parse(profile: string): Profile {
    let tokens = profile.split(";"); // lexer from behringer
    let id = tokens[0];
    let mode1 = tokens[1];
    let opacity1 = tokens[2];
    let data1 = tokens[3];

    let profile = new Profile();
    profile.id = id;
}
