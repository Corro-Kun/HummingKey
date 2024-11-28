<script>
    import {send, receive} from '@/lib/transition.js'
  import toast from 'svelte-french-toast';

    let data = $state([]);

    let check = $state({
        Import: false,
        ValidateImport: false,
        Export: false,
        ValidateExport: false,
    })

    let loading = $state(0);

    let passwords = $state({
        password: "",
        FilePassword: ""
    })

    async function ImportCSV() {
        const { open } = await import('@tauri-apps/api/dialog');
        const { readTextFile} = await import('@tauri-apps/api/fs');

		const filePath = await open({
    		multiple: false,
    		filters: [
      			{ name: 'contraseñas', extensions: ['csv',] }, 
    		],
  		});

        const text = await readTextFile(filePath);
        const rows = text.split('\n');

        if(rows[0].includes('id;name;icon;user;user_length;password;password_length') === false){
            toast.error('Archivo CSV no valido');
            return
        }

        for (let i = 1; i < rows.length; i++) {
            const columns = rows[i].split(';');
            let obj = {};
            
            for (let j = 0; j < columns.length; j++) {
                obj[rows[0].split(';')[j]] = columns[j];
            }
            data.push(obj);
        }

        check.ValidateImport = true;
        toast('Contraseñas importadas, por favor digital la contraseña del archivo sin errores');
    }

    async function ExportCSV(){
        loading = 1;

        const { invoke } = await import('@tauri-apps/api');

        let result = await invoke("login", {password: passwords.password});

		if(!result){
            toast.error('Contraseña incorrecta');
            loading = 0;
            return
		}

        passwords.password = "";

        data = await invoke("get_passwords");

        let dataText = "data:text/csv;charset=utf-8,id;name;icon;user;user_length;password;password_length";

        data.map((item)=>{
            dataText += `\n${Number(item.id)};${item.name};${item.icon};${item.user};${item.user_length};${item.password};${item.password_length}`;
        })

        const encodedUri = encodeURI(dataText);

        const link = document.createElement("a");
        link.setAttribute("href", encodedUri);
        link.setAttribute("download", "contraseñas.csv");

        document.body.appendChild(link);

        link.click();

        document.body.removeChild(link);

        data = [];
        loading = 0;
    }

    async function SavePasswords(){
        const { invoke } = await import('@tauri-apps/api');

        loading = 1;

        let result = await invoke("login", {password: passwords.password});

        if(!result){
            toast.error('Contraseña incorrecta');
            loading = 0;
            return
		}

        data.forEach((item)=>{
            item.id = Number(item.id);
            item.icon = Number(item.icon);
            item.user_length = Number(item.user_length);
            item.password_length = Number(item.password_length);
        })

        try {
            await invoke("import_passwords", {passwords: data, passwordCurrent: passwords.password, filePassword: passwords.FilePassword});
        } catch (error) {
            toast.error('Error al guardar las contraseñas');
            loading = 0;
            return
        } 

        loading = 0;
        toast.success('Contraseñas guardadas');

        data = [];
        passwords.password = "";
        passwords.FilePassword = "";
        check.Import = false;
        check.ValidateImport = false;
    }

</script>

<div class="main" >
    <div class="menu" >
        <button onclick={()=> {
            check.Export = false
            check.Import = !check.Import
            check.ValidateImport = false
            data = [];
        }} >Importar</button>
        <button onclick={()=> {
            check.Import = false
            check.Export = !check.Export
            check.ValidateExport = false
            data = [];
        }} >Exportar</button>
    </div>
    {#if check.Import}
    <div class="import"
    >
        <p>Selecione un archivo CSV</p>
        <button onclick={ImportCSV} >Archivo</button>
        {#if check.ValidateImport}
            <p style="margin-bottom: 5px;" >Contraseñas que se importan: {data.length}</p>
            <div class="input" >
                <label for="">Contraseña del Archivo</label>
                <input type="password" bind:value={passwords.FilePassword} >
            </div>
            <div class="input" >
                <label for="">Tu Contraseña</label>
                <input type="password" bind:value={passwords.password} >
            </div>
            {#if loading === 1}
                <button disabled >Cargando...</button>
            {:else}
                <button onclick={SavePasswords} >Guardar</button>
            {/if}
        {/if}
    </div>
    {/if}
    {#if check.Export}
    <div class="confirm" 
    >
        <h2>Escribe tu contraseña</h2>
        <input type="password"
            bind:value={passwords.password} 
            onkeypress={(e)=> {
                if(e.code === "Enter"){
                    ExportCSV();
                }
            }} >
        <div>
            {#if loading === 1}
                <button disabled >Cargando...</button>
            {:else}
                <button onclick={ExportCSV} >Confirmar</button>
            {/if}
        </div>
    </div>
    {/if}
</div>

<style>
    .main {
        display: flex;
        width: 95%;
        justify-content: space-between;
        align-items: center;
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
    button{
        padding: 8px 15px;
        background: var(--Color_Primary);
        color: var(--Color_Text);
        border: none;
        border-radius: 5px;
        cursor: pointer;
        transition: .3s;
    }
    button:hover{
        background: var(--Color_Secondary);
    }
    .import{
        padding: 10px;
        display: flex;
        width: 300px;
        gap: 10px;
        color: var(--Color_Text);
        align-items: center;
        justify-content: center;
        flex-direction: column;
        background: transparent;
        backdrop-filter: blur(10px);
        border-radius: 10px;
        border: 2px solid var(--Color_Primary);
        transition: .3s;
    }
    .input{
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    .input label{
        color: var(--Color_Text);
    }
    .input input{
        width: 100%;
        margin-top: 2px;
        padding: 2px;
        font-size: 17px;
        background: transparent;
        border: none;
        border-bottom: 2px solid var(--Color_Primary);
        border-radius: 5px;
        outline: none;
        color: var(--Color_Text_Hover);
    }
    .confirm{
        display: flex;
        padding: 10px;
        background: transparent;
        backdrop-filter: blur(10px);
        border: 2px solid var(--Color_Primary);
        border-radius: 8px;
        flex-direction: column;
        align-items: center;
        gap: 10px;
    }
    .confirm h2{
		color: var(--Color_Text);
	}
    .confirm input{
        background: transparent;
		border: none;
        border-bottom: 2px solid var(--Color_Primary);
		outline: none;
		font-size: 1em;
		color: var(--Color_Text);
		font-weight: 600;
		padding: 0 35px 0 5px;
    }
    .confirm div{
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>