<script>
    // import {createGrid} from 'ag-grid-community';
    // import 'ag-grid-community/styles/ag-grid.css';
    // import 'ag-grid-community/styles/ag-theme-quartz.css';

    // const gridOptions = {
    //     columnDefs: [
    //         {headerName: 'Make', field: 'make'},
    //         {headerName: 'Model', field: 'model'},
    //         {headerName: 'Price', field: 'price'},
    //     ],
    //     rowData : [
    //         {make:'Toyota',model:'Celica',price:35000},  
    //         {make:'Ford',model:'Mondeo',price:32000},  
    //         {make:'Porsche',model:'Boxster',price:72000},  
    //     ],
    // }
    
    // const eGridDiv = document.querySelector('#myGrid');
    // const api = createGrid(eGridDiv, gridOptions);

    import {onMount, onDestroy} from "svelte";

    import Handsontable from "handsontable";

    let hot;
    let cont;

    onMount(() => {
        hot = new Handsontable(cont, {
            data : [
                ['','',''],
                ['','',''],
                ['','','']
            ],
        });

        return () => {
            hot.destroy();
        };
    });
</script>

<!-- <div id="myGrid"style = "height:150px;width:600px;" class="ag-theme-quartz"></div> -->
<div bind:this={cont}></div>
<div>{cont}</div>