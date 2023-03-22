<script lang="ts">

var state = 0;
export let color;
export let speed;

let i;
$: {
    clearInterval(i);
    i = setInterval(function() {
        if (state == 360) {
            state = 0;
            return;
        }
        state++;
    }, (1800/speed));
}

</script>
<g
   id="Vrstva_2_00000068639540309316769050000003377542866050839450_"
   transform="matrix(1.2481416,0,0,1.2481416,-114.72913,-105.16066)"   
   >
	<g
   id="Vrstva_1_00000165236326566489921610000009472640422387894949_">
	</g>
	<path
   class="st2"
   style="fill: {color} !important; opacity: {Math.sin(state/100)*100}%;"
   d="m 269.6,292.3 c -4.9,-4.4 -8.9,-2.6 -8.9,4 L 260.6,794 c 0,6.6 -3.8,8.1 -8.4,3.4 l -90,-91.6 c -4.6,-4.7 -8.4,-13.9 -8.4,-20.5 V 158.2 c 0,-6.6 5.4,-12 12,-12 h 82.9 c 6.6,0 15.9,3.6 20.8,8.1 l 212.1,195 c 4.9,4.5 12.4,4.1 16.8,-0.8 L 670.2,155.2 c 4.4,-4.9 13.3,-9 19.9,-9 H 815 c 6.6,0 8.2,3.8 3.6,8.6 L 498.8,482 c -4.6,4.7 -12.4,5 -17.3,0.6 z"
   id="path32" />
</g>
