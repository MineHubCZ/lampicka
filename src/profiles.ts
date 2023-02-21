// TODO rethink this whole thing, consultate with yamiteru

export class Profile {
    id: number;
    opacity1: number;
    opacity2?: number;
    modes: [Mode, Mode] | IndividualGradAll;

    export(): string {
        if (this.modes instanceof IndividualGradAll) {
            return this.id.toString() + this.modes.export().join(";" + this.opacity1.toString() + ";");
        }

        let first = this.modes[0].export().join(";" + this.opacity1.toString() + ";");
        let second = this.modes[1].export().join(";" + this.opacity2.toString() + ";");
        
        return this.id.toString() + ";" + first + ";" + second;
    } 
}

export interface Exportable { 
    export(): [string, string];
}

export interface Mode extends Exportable { }

export class Static implements Mode {
    color: string;
    export(): [string, string] {
        return ["s", this.color];
    }
}

export class Rainbow implements Mode {
    speed: number;
    export(): [string, string] {
        return ["r", this.speed.toString()]
    }
}

export class RainbowGrad extends Rainbow { 
    export(): [string, string] {
        return ["rg", this.speed.toString()];
    }
}

export class Individual implements Mode {
    colors: [string, string, string, string, string] | [string, string, string];
    export(): [string, string] {
        return ["i", this.colors.join(",")];
    }   
}

export class IndividualGrad extends Individual {
    speed: number;
    export(): [string, string] {
        return ["ig", this.speed.toString() + "," + this.colors.join(",")];
    }   
}

export class IndividualGradAll implements Exportable {
    colors: [string, string, string, string, string, string, string, string];
    speed: number;
    export(): [string, string] {
        return ["a", this.speed.toString() + "," + this.colors.join(",")];
    }
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
