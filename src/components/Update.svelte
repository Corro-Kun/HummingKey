<script>
    import { check } from '@tauri-apps/plugin-updater';
    import { relaunch } from '@tauri-apps/plugin-process';
    import {navigate} from 'astro:transitions/client';

    async function handleUpdate() {
        const update = await check();

        await update.downloadAndInstall();
        await relaunch();
    }

    async function handleSkip() {
        const { invoke } = await import('@tauri-apps/api/core');

        let result = await invoke("verify_db");
        if (result){
            navigate("/mainLogin");
        } else {
            navigate("/newUser");
        }
    }
</script>

<div class="main" >
    <h2>Nueva actualización disponible</h2>
    <p>¿Quieres proceder con la instalación de la nueva versión?</p>
    <div class="buttons" >
        <button on:click={handleUpdate} >Actualizar</button>
        <button on:click={handleSkip} >Saltar</button>
    </div>
</div>

<style>
    .main{
        display: flex;
        width: 300px;
        padding: 10px;
        background: transparent;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		overflow: hidden;
		border: 2px solid var(--Color_Primary);
		border-radius: 10px;
        gap: 5px;
		backdrop-filter: blur(20px);
    }
    .main h2{
        color: var(--Color_Text);
        text-align: center;
    }
    .main p{
        color: var(--Color_Text);
    }
    .buttons{
        margin-top: 10px;
        width: 100%;
        display: flex;
        justify-content: space-between;
    }
    .buttons button:nth-child(1){
        padding: 10px;
        border: none;
        border-radius: 5px;
        background: var(--Color_Primary);
        color: var(--Color_Text);
        cursor: pointer;
        transition: .3s;
    }
    .buttons button:nth-child(1):hover{
        color: var(--Color_Text_Hover);
        background: var(--Color_Secondary);
    }
    .buttons button:nth-child(2){
        padding: 10px;
        border: none;
        background: transparent;
        color: var(--Color_Text);
        cursor: pointer;
    }
</style>