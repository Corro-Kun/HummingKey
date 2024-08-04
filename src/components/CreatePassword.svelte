<script>
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
</script>
<form>
    <div class="title" >
        <h1>Crea tu nueva contraseña</h1>
    </div>
    <div class="input" >
        <label for="name">Nombre de tu contraseña</label>
        <input type="text" id="name" required >
    </div>
    <div class="icon" >
        <h2>Selecciona un icono</h2>
        <div class="icons" >
            <section>
                <Box />
            </section>
            <section>
                <Book />
            </section>
            <section>
                <Store />
            </section>
            <section>
                <Work />
            </section>
            <section>
                <Game />
            </section>
            <section>
                <Cloudy />
            </section>
            <section>
                <Google />
            </section>
            <section>
                <Windows />
            </section>
            <section>
                <Github />
            </section>
            <section>
                <Facebook />
            </section>
            <section>
                <X />
            </section>
            <section>
                <Reddit />
            </section>
            <section>
                <Spotify />
            </section>
            <section>
                <Netflix />
            </section>
            <section>
                <Discord />
            </section>
            <section>
                <Steam />
            </section>
        </div>
    </div>
    <div class="input" >
        <label for="">Correo/usuario</label>
        <input type="text" required >
    </div>
    <div class="input" >
        <label for="">Contraseña</label>
        <input type="password" required >
    </div>
    <div class="button-save" >
        <button>Guardar</button>
    </div>
</form>

<style>
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
    .button-save{
        display: flex;
        justify-content: center;
        margin-block: 15px;
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