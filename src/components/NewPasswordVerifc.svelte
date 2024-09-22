<script>
    import {request, newPassword} from '@/store/password.ts';
    import {navigate} from 'astro:transitions/client';
    import toast from "svelte-french-toast";

    request.set(false);
    newPassword.set({});

    let password = "";

    async function save() {
        const { invoke } = await import('@tauri-apps/api');
        
        let result = await invoke("login", {password: password});

		if(!result){
            toast.error('Contraseña incorrecta');
            return
		}

        let result2 = await invoke("create_password", {newPassword: $newPassword, password: password});

        if(!result2){
            // trabajando en esto
        }

        request.set(false);
        newPassword.set({});

        navigate("/home");
    }

</script>

{#if !$request}
<h1></h1>
{:else if $request}
<div class="confirm" >
    <h2>Escribe tu contraseña</h2>
    <input bind:value={password} type="password">
    <div>
        <button on:click={save} >Confirmar</button>
    </div>
</div>
{/if}

<style>
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
    h2{
		color: var(--Color_Text);
	}
    input{
        background: transparent;
		border: none;
        border-bottom: 2px solid var(--Color_Primary);
		outline: none;
		font-size: 1em;
		color: var(--Color_Text);
		font-weight: 600;
		padding: 0 35px 0 5px;
    }
    div{
        display: flex;
        justify-content: center;
        align-items: center;
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
</style>