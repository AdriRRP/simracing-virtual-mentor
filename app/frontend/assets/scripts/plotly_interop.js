export function sync_plotly(div_id, sync_div_ids) {
    let dashboardPlot = document.getElementById(div_id);
    let plotsNames = [...Object.keys(dashboardPlot._fullLayout._plots)];
    dashboardPlot.on('plotly_hover', function (event) {

        // Emitir evento con el índice de distancia
        let distanceIndex = event.points[0].pointIndex;
        let customEvent = new CustomEvent("update_suggestion_event", {
            detail: distanceIndex
        });
        document.dispatchEvent(customEvent);

        sync_div_ids.forEach(sync_div_id => {
            let syncPlot = document.getElementById(sync_div_id);
            Plotly.Fx.hover(
                syncPlot,
                { xval: event.xvals[0] },
                plotsNames
            );
            window.wasmBindings.hover_event_from_plotly(event.xvals[0]);

        });
    });

    //dashboardPlot.on('plotly_unhover', function (event) {
    //    sync_div_ids.forEach(sync_div_id => {
    //        let syncPlot = document.getElementById(sync_div_id);
    //        Plotly.Fx.unhover(syncPlot, {});
    //    });
    //});

    dashboardPlot.on("plotly_relayout", function(ed) {
        sync_div_ids.forEach((div_id, i) => {
            let div = document.getElementById(div_id);
            let x = div.layout.xaxis;
            if (ed["xaxis.autorange"] && x.autorange) return;
            if (
                x.range[0] !== ed["xaxis.range[0]"] ||
                x.range[1] !== ed["xaxis.range[1]"]
            )
                Plotly.relayout(div, ed);
        });
    });

}

export function updatePlotlyHover(divIds, point) {
    divIds.forEach(id => {
        let chart = document.getElementById(id);
        let plotsNames = [...Object.keys(chart._fullLayout._plots)];

        Plotly.Fx.hover(
            chart,
            { xval: point.dist },
            plotsNames
        );
    });
}