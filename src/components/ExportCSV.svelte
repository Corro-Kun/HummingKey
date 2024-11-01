<script>
    let data = [];
    async function ImportCSV() {
        const { open } = await import('@tauri-apps/api/dialog');
        const { readTextFile } = await import('@tauri-apps/api/fs');

		const filePath = await open({
    		multiple: false,
    		filters: [
      			{ name: 'Copia', extensions: ['csv',] }, 
    		],
  		});

        const text = await readTextFile(filePath);
        const rows = text.split('\n');

        for (let i = 1; i < rows.length; i++) {
            const columns = rows[i].split(';');
            let obj = {};
            
            for (let j = 0; j < columns.length; j++) {
                obj[rows[0].split(';')[j]] = columns[j];
            }
            data.push(obj);
        }
    }

    function ExportCSV() {
        var data = "Nombre;Edad\nJuan;30\nAna;25";
        var uriContent = "data:text/csv;charset=utf-8," + encodeURIComponent(data);
        var myWindow = window.open(uriContent, "Export CSV");
        myWindow.focus();
    }
</script>

<div class="main" >
    <div class="menu" >
        <button on:click={ExportCSV} >Exportar</button>
        <button on:click={ImportCSV} >Importar</button>
    </div>
</div>

<style>
    .main {
        display: flex;
        width: 95%;
    }
    .menu{
        display: flex;
        width: 200px;
        height: 200px;
        gap: 20px;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        border-radius: 10px;
        border: 2px solid var(--Color_Primary);
        background: transparent;
        backdrop-filter: blur(5px);
    }
    .menu button{
        padding: 8px 15px;
        background: var(--Color_Primary);
        color: var(--Color_Text);
        border: none;
        border-radius: 5px;
        cursor: pointer;
        transition: .3s;
    }
    .menu button:hover{
        background: var(--Color_Secondary);
    }
</style>