<script lang="ts">

// pls forgive me

let color1;
let color2;
let color3;
let color4;
let color5;

export let color;

var c = 0;

export let speed;

let i;
$: {
    clearInterval(i);
    i = setInterval(function() {
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
                    color3 = color;
                    break;           
                case 100: 
                    color2 = color;
                    break;
                case 125: 
                    c = 0;
                    return;
            }
            c++;
        }, (180/speed));
}

</script>
<defs>
    <linearGradient id="bottomGradient">
        <stop offset="0%" stop-color="{color3}">
        </stop>
        <stop offset="50%" stop-color="{color2}">
        </stop>
        <stop offset="100%" stop-color="{color1}">
        </stop>
    </linearGradient>
</defs>
<g
   id="Vrstva_3"
   transform="matrix(1.2481416,0,0,1.2481416,-114.72913,-105.16066)"
   >
    <g
   id="Vrstva_2_00000068674944005382951290000011903351550368725390_"
   class="st3">
	</g>
	<path
   fill="url(#bottomGradient)"
   d="m 825.4,816.1 c 0,6.6 -5.4,12 -12,12 h -76.6 c -6.6,0 -12,-5.4 -12,-12 V 547.7 c 0,-6.6 -3.8,-8.2 -8.4,-3.5 L 496.5,767 c -4.6,4.7 -12.2,4.7 -16.8,0 L 351,637.5 c -4.7,-4.7 -8.4,-13.8 -8.4,-20.5 V 498 c 0,-6.6 3.8,-8.2 8.4,-3.5 L 479.6,624 c 4.7,4.7 12.2,4.7 16.8,0 L 817,298.3 c 4.6,-4.7 8.4,-3.2 8.4,3.4 z"
   id="path36" />
</g>
