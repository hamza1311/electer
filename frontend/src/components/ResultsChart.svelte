<script>
    import FusionCharts from 'fusioncharts';
    import Charts from 'fusioncharts/fusioncharts.charts';
    import FusionTheme from 'fusioncharts/themes/fusioncharts.theme.fusion';
    import SvelteFC, {fcRoot} from 'svelte-fusioncharts';
    import {onMount} from "svelte";

    export let title;
    export let results;

    const dataSource = {
        "chart": {
            "caption": `Results for ${title}`,
            "showValues": "1",
            "showPercentInTooltip": "0",
            "theme": "fusion"
        },
        "data": results
    }

    fcRoot(FusionCharts, Charts, FusionTheme);

    let chartConfig = {
        type: 'pie3d',
        width: '600',
        height: '400',
        renderAt: 'chart-container',
        dataSource
    };

    onMount(() => {

    })
</script>

<div id="chart-container">
    <SvelteFC {...chartConfig}/>
</div>

