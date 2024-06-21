//export function sync_plotly_hover(div_id) {
//    var dashboardPlot = document.getElementById(div_id);
//    var plotsNames = [...Object.keys(dashboardPlot._fullLayout._plots)];
//    dashboardPlot.on('plotly_hover', function (file) {
//        Plotly.Fx.hover(
//            dashboardPlot,
//            { xval: file.xvals[0] },
//            plotsNames
//        );
//    });
//}

export function sync_plotly_hover(div_id, sync_div_ids) {
    var dashboardPlot = document.getElementById(div_id);
    var plotsNames = [...Object.keys(dashboardPlot._fullLayout._plots)];
    dashboardPlot.on('plotly_hover', function (event) {
        sync_div_ids.forEach(sync_div_id => {
            var syncPlot = document.getElementById(sync_div_id);
            Plotly.Fx.hover(
                syncPlot,
                { xval: event.xvals[0] },
                plotsNames
            );
        });
    });

    dashboardPlot.on('plotly_unhover', function (event) {
        sync_div_ids.forEach(sync_div_id => {
            var syncPlot = document.getElementById(sync_div_id);
            Plotly.Fx.unhover(syncPlot, {});
        });
    });

    dashboardPlot.on("plotly_relayout", function(ed) {
        sync_div_ids.forEach((div_id, i) => {
            let div = document.getElementById(div_id);
            let x = div.layout.xaxis;
            if (ed["xaxis.autorange"] && x.autorange) return;
            if (
                x.range[0] != ed["xaxis.range[0]"] ||
                x.range[1] != ed["xaxis.range[1]"]
            )
                Plotly.relayout(div, ed);
        });
    });

}