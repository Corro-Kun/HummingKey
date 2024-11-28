<script>
    import {onMount} from 'svelte';
    import {profileImg} from '@/store/profile.ts'
    import Loader from '@/components/Loader.svelte';
    import toast from "svelte-french-toast";
    import {send, receive} from '@/lib/transition.js'

    let name = $state("");
    let password = $state("");
    let newPassword = $state({
        password: "",
        confirmPassword: ""
    }) 

    let loading = $state(0);

    let check = $state(false);

    onMount(async ()=>{
        const { invoke } = await import('@tauri-apps/api');
        name = await invoke("get_name_user");
    });

    async function uploadImage(){
		const {convertFileSrc} = await import('@tauri-apps/api/tauri');
		const { open } = await import('@tauri-apps/api/dialog');

		const filePath = await open({
    		multiple: false,
    		filters: [
      			{ name: 'Image', extensions: ['png', 'jpg','jpeg', 'gif'] }, 
    		],
  		});

		if (filePath !== null){
			profileImg.set(convertFileSrc(filePath));
		}
	}

    async function HandleSubmit(e){
        e.preventDefault();

        if (check){
            if (newPassword.password !== newPassword.confirmPassword){
                toast.error('Las contraseñas no coinciden');
                return
            }
        }

        loading = 1;
    }

    async function save(){
        loading = 2;

        const { invoke } = await import('@tauri-apps/api');
        
        let result = await invoke("login", {password: password});

		if(!result){
            toast.error('Contraseña incorrecta');
            loading = 0;
            password = "";
            return
		}

        if (check){
            await invoke("update_user_with_password", {user:{
                name: name,
                image: $profileImg,
                password: newPassword.password
            }, passwordNow: password});
        }else{
            await invoke("update_user_without_password", {user:{
                name: name,
                image: $profileImg,
                password: ""
            }});
        }

        loading = 0;
        password = "";
	newPassword.password = "";
	newPassword.confirmPassword = "";


        toast.success('Usuario actualizado con éxito');
    }

</script>

<div class="main" >
    <div>
        {#if loading}
        <div class="confirm" 
            in:receive
            out:send
        >
            <h2>Escribe tu contraseña</h2>
            <input bind:value={password} type="password" 
                onkeypress={(e)=> {
                    if(e.code === "Enter"){
                        save();
                    }
                }} >
            <div>
                {#if loading === 2}
                    <button disabled >Cargando...</button>
                {:else}
                    <button onclick={save} >Confirmar</button>
                {/if}
            </div>
        </div>
        {/if}
    </div>
    <form class="new" onsubmit={HandleSubmit} >
        <h2>Tu usuario</h2>
        <p>Actualiza tu usuario</p>
        <picture onclick={uploadImage} >
            <img src={$profileImg} alt="profile" loading="lazy" >
        </picture>
        <div class="Inputs" >
            <input bind:value={name} id="user" type="text" autoComplete="off" required >
            <label for="user">Usuario</label>
        </div>
        <div class="CheckBox" >
            <input onchange={()=> check = !check} type="checkbox">
            <label for="">¿Actualizar Contraseña?</label>
        </div>
        {#if check}
        <div class="Inputs"
        >
            <input bind:value={newPassword.password} id="pass" type="password" autoComplete="off" required />
            <label for="pass">Nueva Contraseña</label>
        </div>
        <div class="Inputs"
        >
            <input bind:value={newPassword.confirmPassword} id="pass" type="password" autoComplete="off" required />
            <label for="pass">Confirmar Contraseña</label>
        </div>
        {/if}
        <div class="Button" >
            <button disabled={loading} >
                {#if !loading}
                    Actualizar
                {:else}
                    <Loader />
                {/if}
            </button>
        </div>
    </form>
</div>

<style>
    .main{
		display: flex;
		padding: 50px;
		width: 100%;
		height: 80vh;
		justify-content: space-between;
		align-items: center;
	}
    .new{
		display: flex;
        padding: 20px 0;
		width: 330px;
		background: transparent;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		overflow: hidden;
		border: 2px solid var(--Color_Primary);
		border-radius: 10px;
		backdrop-filter: blur(20px);
        transition: .3s;
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
        transition: .2s;
	}
    .new picture:hover{
        border: 2px solid var(--Color_Secondary);
    }
	.new picture img{
		height: 140px;
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
    .CheckBox{
        display: flex;
        margin-block: 10px;
        width: 85%;
        align-items: center;
        gap: 5px;
    }
    .CheckBox label{
        font-size: 1em;
        color: var(--Color_Text);
        font-weight: 600;
    }
    .CheckBox input[type="checkbox"]{
        cursor: pointer;
        accent-color: var(--Color_Primary);
        outline: none;
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
    .confirm button{
        padding: 8px 15px;
        background: var(--Color_Primary);
        color: var(--Color_Text);
        border: none;
        border-radius: 5px;
        cursor: pointer;
        transition: .3s;
    }
    .confirm button:hover{
        background: var(--Color_Secondary);
    }
</style>