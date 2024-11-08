<script>
    let data = [{
        Nombre: 'Juan',
        Edad: 30
    }, {
        Nombre: 'Ana',
        Edad: 25
    }];


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

    async function ExportCSV(){
        const { invoke } = await import('@tauri-apps/api');
        data = await invoke("get_passwords");

        console.log(data);

        let dataText = "data:text/csv;charset=utf-8,id;name;icon;user;user_length;password;password_length\n";

        data.map((item)=>{
            dataText += `${item.id};${item.name};${item.icon};${item.user};${item.user_length};${item.password};${item.password_length}\n`;
        })

        const encodedUri = encodeURI(dataText);

        const link = document.createElement("a");
        link.setAttribute("href", encodedUri);
        link.setAttribute("download", "contraseñas.csv");

        document.body.appendChild(link);

        link.click();

        document.body.removeChild(link);

        data = [];
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