<script lang="ts">
import { rainbow } from "/src/colors";

// pls forgive me

let color1;
let color2;
let color3;
let color4;
let color5;

var red = 255; 
var green = 0;
var blue = 0;

var c = 0;

export let speed;

let i;
$: {
    clearInterval(i);
    i = setInterval(function() {
            [red, green, blue] = rainbow(red, green, blue);
            let color = `rgb(${red}, ${green}, ${blue})`;
            if (c % 25 == 0) {
                color1 = "#000000";
                color2 = "#000000";
                color3 = "#000000";
                color4 = "#000000";
                color5 = "#000000";
            }
            switch(c) {
                case 0: 
                    color1 = color;
                    break;
                case 25: 
                    color2 = color;
                    break;
                case 50:
                    color3 = color;
                    break;
                case 75:
                    color4 = color;
                    break;
                case 100:
                    color5 = color;
                    break;
                case 125:
                    color4 = color;
                    break;
                case 150:
                    color3 = color;
                    break;           
                case 175: 
                    color2 = color;
                    break;
                case 200: 
                    c = 0;
                    return;
            }
            c++;
        }, (180/speed));
}

</script>
<defs>
    <linearGradient id="gradient">
        <stop offset="0%" stop-color="{color1}">
        </stop>
        <stop offset="25%" stop-color="{color2}">
        </stop>
        <stop offset="50%" stop-color="{color3}">
        </stop>
        <stop offset="75%" stop-color="{color4}">
        </stop>
        <stop offset="100%" stop-color="{color5}">
        </stop>
    </linearGradient>
</defs>
<g
   id="Vrstva_2_00000068639540309316769050000003377542866050839450_"
   transform="matrix(1.2481416,0,0,1.2481416,-114.72913,-105.16066)"   
   >
	<g
   id="Vrstva_1_00000165236326566489921610000009472640422387894949_">
	</g>
	<path
   fill="url(#gradient)"
   d="m 269.6,292.3 c -4.9,-4.4 -8.9,-2.6 -8.9,4 L 260.6,794 c 0,6.6 -3.8,8.1 -8.4,3.4 l -90,-91.6 c -4.6,-4.7 -8.4,-13.9 -8.4,-20.5 V 158.2 c 0,-6.6 5.4,-12 12,-12 h 82.9 c 6.6,0 15.9,3.6 20.8,8.1 l 212.1,195 c 4.9,4.5 12.4,4.1 16.8,-0.8 L 670.2,155.2 c 4.4,-4.9 13.3,-9 19.9,-9 H 815 c 6.6,0 8.2,3.8 3.6,8.6 L 498.8,482 c -4.6,4.7 -12.4,5 -17.3,0.6 z"
   id="path32" />
</g>
