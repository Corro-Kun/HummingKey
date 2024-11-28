<script>
    import { onMount } from 'svelte';
    import {navigate} from 'astro:transitions/client';
    import toast from "svelte-french-toast";
    import Box from '@/components/icons/account/box.svelte';
    import Book from '@/components/icons/account/books.svelte';
    import Store from '@/components/icons/account/store.svelte';
    import Work from '@/components/icons/account/work.svelte';
    import Game from '@/components/icons/account/games.svelte';
    import Cloudy from '@/components/icons/account/cloudy.svelte';
    import Google from '@/components/icons/account/google.svelte';
    import Windows from './icons/account/windows.svelte';
    import Github from '@/components/icons/account/github.svelte';
    import Facebook from '@/components/icons/account/facebook.svelte';
    import Reddit from '@/components/icons/account/reddit.svelte';
    import X from '@/components/icons/account/X.svelte';
    import Spotify from '@/components/icons/account/spotify.svelte';
    import Netflix from '@/components/icons/account/netflix.svelte';
    import Discord from '@/components/icons/account/discord.svelte';
    import Steam from '@/components/icons/account/steam.svelte';
    import Loading from '@/components/icons/Loading.svelte';

    let id = 0;

    let password = $state("");

    let loading = $state(0);

    let confirm = $state(0);

    let data = $state({
        id: 0,
        icon: 0,
        name: "",
        user: "",
        user_length: 0,
        password: "",
        password_length: 0,
    });

    onMount(async () => {
        let url = new URL(window.location.href);
        id = url.searchParams.get('id');
    });

    async function getPassword(){
        loading = 1;
        const { invoke } = await import('@tauri-apps/api');

        let result = await invoke("login", {password: password});

        if(!result){
            toast.error('Contraseña incorrecta');
            loading = 0;
            return
		}

        data = await invoke('get_password_by_id', {id: Number(id), password: password});

        data.password = data.password.substring(data.password_length, 0);
        data.user = data.user.substring(data.user_length, 0);

        loading = 0;
        confirm = 1;
    }

    async function save(e){
        e.preventDefault();
        const { invoke } = await import('@tauri-apps/api');

        loading = 1;

        if(data.name === "" || data.user === "" || data.password === ""){
            toast.error('Todos los campos son obligatorios');
            loading = 0;
            return
        }

        if(data.icon === 0){
            data.icon = 1;
        }

        data.user_length = data.user.length;
        data.password_length = data.password.length;

        await invoke("update_password", {updatePassword: data, password: password});

        toast.success('Contraseña actualizada correctamente');

        loading = 0;

        navigate("/home");
    }
</script>

{#if confirm === 0}
<div class="confirm-password" >
    <h2>Confirma tu Contraseña</h2>
    <input type="password" bind:value={password} >
    <div>
        <button class="cancel" onclick={()=> navigate("/home")} >Cancelar</button>
        {#if loading === 0}
            <button class="confirm" onclick={getPassword} >Confirmar</button>
        {:else if loading === 1}
            <button class="cancel" >
                <Loading />
            </button>
        {/if}
    </div>
</div>
{:else}
<form onsubmit={save} >
    <div class="title" >
        <h1>Actualiza tu nueva contraseña</h1>
    </div>
    <div class="input" >
        <label for="name">Nombre de tu contraseña</label>
        <input type="text" id="name" autocomplete="off" required bind:value={data.name} >
    </div>
    <div class="icon" >
        <h2>Selecciona un icono</h2>
        <div class="icons" >
            <section class={data.icon === 1 ? 'active' : ''} onclick={()=> data.icon = 1} >
                <Box />
            </section>
            <section class={data.icon === 2 ? 'active' : ''} onclick={()=> data.icon = 2} >
                <Book />
            </section>
            <section class={data.icon === 3 ? 'active' : ''} onclick={()=> data.icon = 3} >
                <Store />
            </section>
            <section class={data.icon === 4 ? 'active' : ''} onclick={()=> data.icon = 4} >
                <Work />
            </section>
            <section class={data.icon === 5 ? 'active' : ''} onclick={()=> data.icon = 5} >
                <Game />
            </section>
            <section class={data.icon === 6 ? 'active' : ''} onclick={()=> data.icon = 6} >
                <Cloudy />
            </section>
            <section class={data.icon === 7 ? 'active' : ''} onclick={()=> data.icon = 7} >
                <Google />
            </section>
            <section class={data.icon === 8 ? 'active' : ''} onclick={()=> data.icon = 8} >
                <Windows />
            </section>
            <section class={data.icon === 9 ? 'active' : ''} onclick={()=> data.icon = 9} >
                <Github />
            </section>
            <section class={data.icon === 10 ? 'active' : ''} onclick={()=> data.icon = 10} >
                <Facebook />
            </section>
            <section class={data.icon === 11 ? 'active' : ''} onclick={()=> data.icon = 11} >
                <X />
            </section>
            <section class={data.icon === 12 ? 'active' : ''} onclick={()=> data.icon = 12} >
                <Reddit />
            </section>
            <section class={data.icon === 13 ? 'active' : ''} onclick={()=> data.icon = 13} >
                <Spotify />
            </section>
            <section class={data.icon === 14 ? 'active' : ''} onclick={()=> data.icon = 14} >
                <Netflix />
            </section>
            <section class={data.icon === 15 ? 'active' : ''} onclick={()=> data.icon = 15} >
                <Discord />
            </section>
            <section class={data.icon === 16 ? 'active' : ''} onclick={()=> data.icon = 16} >
                <Steam />
            </section>
        </div>
    </div>
    <div class="input" >
        <label for="">Correo/usuario</label>
        <input type="text" required bind:value={data.user} >
    </div>
    <div class="input" >
        <label for="">Contraseña</label>
        <input type="password" required bind:value={data.password} >
    </div>
    <div class="button-save" >
        {#if loading === 0}
            <button>Actualizar</button>
        {:else if loading === 1}
            <Loading />
        {/if}
    </div>
</form>
{/if}

<style>
    .confirm-password {
        display: flex;
        padding: 10px;
        background: transparent;
        backdrop-filter: blur(10px);
        border: 2px solid var(--Color_Primary);
        border-radius: 8px;
        flex-direction: column;
        align-items: center;
        gap: 20px;
    }
    .confirm-password h2{
        color: var(--Color_Text);
    }
    .confirm-password input{
        background: transparent;
		border: none;
        border-bottom: 2px solid var(--Color_Primary);
		outline: none;
		font-size: 1em;
		color: var(--Color_Text);
		font-weight: 600;
		padding: 0 35px 0 5px;
    }
    .confirm-password div{
        width: 100%;
        display: flex;
        justify-content: space-between;
    }
    .cancel{
        background: none;
        border: none;
        cursor: pointer;
        color: var(--Color_Text);
    }
    .confirm{
        padding: 8px 15px;
        background: var(--Color_Primary);
        color: var(--Color_Text);
        border: none;
        border-radius: 5px;
        cursor: pointer;
        transition: .3s;
    }
    .confirm:hover{
        background: var(--Color_Secondary);
    }
    form{
        height: 500px;
        width: 500px;
        background: transparent;
        backdrop-filter: blur(10px);
        border: 2px solid var(--Color_Primary);
        border-radius: 8px;
    }
    .title{
        margin-block: 10px;
        display: flex;
        justify-content: center;
        font-size: 12px;
        color: var(--Color_Text);
    }
    .input{
        margin-block: 10px;
        padding: 10px;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    .input label{
        font-size: 20px;
        color: var(--Color_Text);
    }
    .input input{
        width: 70%;
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
    .icon{
        padding-inline: 10px;
        width: 100%;
        margin-block: 5px;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    .icon h2{
        margin-bottom: 10px;
        font-size: 20px;
        color: var(--Color_Text);
    }
    .icons{
        display: flex;
        gap: 8px;
        width: 100%;
        flex-wrap: wrap;
    }
    .icons section{
        padding: 10px 12px 8px;
        width: max-content;
        border: 2px solid var(--Color_Primary);
        border-radius: 5px;
        cursor: pointer;
        color: var(--Color_Text_Hover);
        transition: .2s;
    }
    .icons section:hover{
        background: var(--Color_Primary);
    }
    .active{
        background: var(--Color_Primary);
    }
    .button-save{
        display: flex;
        justify-content: center;
        margin-block: 15px;
        color: var(--Color_Text);
    }
    .button-save button{
        padding: 8px 15px;
        background: var(--Color_Primary);
        color: var(--Color_Text);
        border: none;
        border-radius: 5px;
        cursor: pointer;
        transition: .3s;
    }
    .button-save button:hover{
        background: var(--Color_Secondary);
    }
</style>