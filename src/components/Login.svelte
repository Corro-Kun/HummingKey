<script>
	import {onMount} from 'svelte';
	import {navigate} from 'astro:transitions/client';
    import toast from 'svelte-french-toast';
    import { error } from 'node_modules/astro/dist/core/logger/core';

	let user = "";

	let password = "";

	let loading = false;

	onMount(async ()=>{
		const { invoke } = await import('@tauri-apps/api');
		user = await invoke("get_name_user");
	});

    async function HandleSubmit(e) {
        e.preventDefault();
		loading = true;
		const { invoke } = await import('@tauri-apps/api');

		let result = await invoke("login", {password: password});

		if(!result){
			loading = false
			password = ""
			throw new Error()
		}

		navigate("/home");
    }
</script>

<form class="login" on:submit={(e)=> toast.promise(HandleSubmit(e),{
	loading: 'Iniciando sesión...',
	success: `Bienvenido ${user}`,
	error: 'Contraseña incorrecta'
})} >
    <picture>
        <img src="https://somoskudasai.com/wp-content/uploads/2022/10/portada_ia-4.jpg" alt="profile" loading="lazy" >
    </picture>
    	<h2>{user}</h2>
	{#if !loading}
    <div class="password">
        <input bind:value={password} id="pass" type="password" autoComplete="off" required />
        <label for="pass">Contraseña</label>
    </div>
	{/if}
</form>

<style>
    .login{
		display: flex;
		width: 300px;
		height: 300px;
		background: transparent;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		overflow: hidden;
		border: 2px solid var(--Color_Primary);
		border-radius: 10px;
		backdrop-filter: blur(20px);
	}
	.login picture{
		display: flex;
		width: 140px;
		height: 140px;
		align-items: center;
		justify-content: center;
		overflow: hidden;
		border-radius: 50%;
        border: 2px solid var(--Color_Primary);
		transition: .2s;
	}
	.login picture img{
		height: 145px;
	}
	.login h2{
		margin-top: 20px;
		margin-bottom: 14px;
		font-size: 1.8em;
		color: var(--Color_Text);
	}
	.password{
		position: relative;
		width: 85%;
		height: 32px;
		border-bottom: 2px solid var(--Color_Primary);
	}	
	.password label{
		position: absolute;	
		top: 50%;
		left: 5px;
		transform: translateY(-50%);
		font-size: 1em;
		color: var(--Color_Text);
		font-weight: 600;
		pointer-events: none;
		transition: .4s;
	}
	.password input{
		width: 100%;
		height: 100%;
		background: transparent;
		border: none;
		outline: none;
		font-size: 1em;
		color: var(--Color_Text);
		font-weight: 600;
		padding: 0 35px 0 5px;
		transition: .2s;
	}
	.password input:focus~label,
	.password input:valid~label{
		top: -5px;
		font-size: .8em;
		color: var(--Color_Primary);
	}
</style>