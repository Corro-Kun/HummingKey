<script>
	import {navigate} from 'astro:transitions/client';
	import Loader from '@/components/Loader.svelte';

	let data = {
		name: "",
		password: ""
	}

	let loading = false;

	async function HandleSubmit(e){
		e.preventDefault();

		loading = true;

		const { invoke } = await import('@tauri-apps/api');

		await invoke("create_user", {user: data});

		navigate("/mainLogin");
	}
</script>

<form class="new" on:submit={HandleSubmit} >
    <h2>Bienvenido a HummingKey</h2>
    <p>Crea tu usuario y credenciales</p>
    <picture>
        <img src="https://somoskudasai.com/wp-content/uploads/2022/10/portada_ia-4.jpg" alt="profile" loading="lazy" >
    </picture>
    <div class="Inputs" >
        <input bind:value={data.name} id="user" type="text" required>
        <label for="user">Usuario</label>
    </div>
    <div class="Inputs">
        <input bind:value={data.password} id="pass" type="password" required />
        <label for="pass">Contraseña</label>
    </div>
    <div class="Button" >
        <button disabled={loading} >
			{#if loading}
				<Loader />
			{:else}
				Crear
			{/if}
		</button>
    </div>
</form>

<style>
    .new{
		display: flex;
		width: 330px;
		height: 410px;
		background: transparent;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		overflow: hidden;
		border: 2px solid var(--Color_Primary);
		border-radius: 10px;
		backdrop-filter: blur(20px);
	}
	.new picture{
		display: flex;
		width: 140px;
		height: 140px;
        margin-block: 15px;
		align-items: center;
		justify-content: center;
		overflow: hidden;
		border-radius: 50%;
        border: 2px solid var(--Color_Primary);
        cursor: pointer;
	}
	.new picture img{
		height: 145px;
	}
	.new h2{
		font-size: 1.4em;
		color: var(--Color_Text);
	}
    .new p{
        font-size: 1.2em;
        color: var(--Color_Text);
    }
	.Inputs{
        margin-block: 10px;
		position: relative;
		width: 85%;
		height: 32px;
		border-bottom: 2px solid var(--Color_Primary);
	}	
	.Inputs label{
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
	.Inputs input{
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
	.Inputs input:focus~label,
	.Inputs input:valid~label{
		top: -5px;
		font-size: .8em;
		color: var(--Color_Primary);
	}
    .Button{
        display: flex;
        width: 85%;
        margin-top: 20px;
        align-items: center;
        justify-content: center;
    }
    .Button button{
        width: 100%;
		height: 35px;
		display: flex;
		align-items: center;
		justify-content: center;
        background: var(--Color_Primary);
        color: var(--Color_Text);
        font-size: 1em;
        font-weight: 600;
        border: none;
        outline: none;
        cursor: pointer;
        border-radius: 5px;
        transition: .2s;
    }
    .Button button:hover{
        background: var(--Color_Secondary);
    }
</style>