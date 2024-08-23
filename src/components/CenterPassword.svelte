<script>
    import Trash from '@/components/icons/Trash.svelte';
    import Pencil from '@/components/icons/Pencil.svelte';
    import AccountIcons from '@/components/AccountIcons.svelte';
    import Copy from '@/components/icons/Copy.svelte';
    import Eye from '@/components/icons/Eye.svelte';
</script>

<div class="card" >
    <div class="list" >
        <div class="title" >
            <h2>Contraseñas</h2>
        </div>
        <div class="list-div" >
            <div>
                <AccountIcons icon={9} />
                <p class="text-pass" >Cuenta de github</p>
            </div>
            <div>
                <p>-</p>
                <AccountIcons icon={13} />
                <p class="text-pass" >Spotify</p>
            </div>
            <div>
                <AccountIcons icon={7} />
                <p class="text-pass" >Cuenta de celular</p>
            </div>
        </div>
    </div>
    <div class="content" >
        <div class="left" >
            <div class="title-content" >
                <AccountIcons icon={13} />
                <h2>Spotify</h2>
            </div>
            <div class="passwords" >
                <div class="pass-content" >
                    <div>
                        <h3>Email</h3>
                        <p>*********</p>
                    </div>
                    <div>
                        <button>
                            <Eye />
                        </button>
                        <button>
                            <Copy />
                        </button>
                    </div>
               </div>
                <div class="pass-content" >
                    <div>
                        <h3>Contraseña</h3>
                        <p>*********</p>
                    </div>
                    <div>
                        <button>
                            <Eye />
                        </button>
                        <button>
                            <Copy />
                        </button>
                    </div>
               </div>
            </div>
        </div>
        <div class="right" >
            <button><Pencil /></button>
            <button><Trash /></button> 
        </div>
    </div>
</div>

<style>
    .card{
        display: flex;
        padding: 0 10px;
        height: 300px;
        width: 100%;
        align-items: center;
        justify-content: space-between;
    }
    .list{
        height: 300px;
        width: 250px;
        background: transparent;
        backdrop-filter: blur(10px);
        border: 2px solid var(--Color_Primary);
        border-radius: 8px;
        overflow: hidden;
    }
    .title{
        height: 30px;
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        border-bottom: 2px solid var(--Color_Primary);
    }
    .title h2{
        color: var(--Color_Text);
    }
    .list-div{
        display: flex;
        height: 270px;
        width: 100%;
        overflow-x: hidden;
        overflow-y: auto;
        flex-direction: column;
    }
    .list-div::-webkit-scrollbar{
        background: transparent;
        width: 7px;
    }
    .list-div::-webkit-scrollbar-thumb{
        background: var(--Color_Primary);
        border-radius: 8px;
    }
    .list-div::-webkit-scrollbar-track{
        background: transparent;
    }
    .list-div div{
        display: flex;
        margin-top: 5px;
        padding: 2px 5px;
        cursor: pointer;
        transition: .2s;
        align-items: center;
        color: var(--Color_Text);
        gap: 10px;
        overflow: hidden;
    }
    .list-div div:hover{
        color: var(--Color_Text_Hover);
    }
    .text-pass{
        font-size: 18px;
        transition: .2s;
    }
    .content{
        display: flex;
        padding: 4px 10px;
        height: 250px;
        width: 400px;
        background: transparent;
        backdrop-filter: blur(10px);
        border: 2px solid var(--Color_Primary);
        border-radius: 8px;
        overflow: hidden;
    }
    .title-content{
        display: flex;
        height: 30px;
        width: 100%;
        align-items: center;
        gap: 10px;
        color: var(--Color_Text);
    }
    .passwords{
        display: flex;
        padding: 0 10px;
        height: 140px;
        width: 100%;
        justify-content: center;
        flex-direction: column;
    }
    .passwords h3{
        color: var(--Color_Text);
    }
    .passwords p{
        color: var(--Color_Text_Hover);
    }
    .passwords button{
        background: transparent;
        border: none;
        cursor: pointer;
        color: var(--Color_Text);
        transition: .2s;
    }
    .passwords button:hover{
        color: var(--Color_Secondary);
    }
    .pass-content{
        display: flex;
        align-items: center;
        justify-content: space-between;
    }
    .left{
        height: 100%;
        width: 90%;
    }
    .right{
        display: flex;
        padding: 8px 0;
        height: 100%;
        width: 10%;
        align-items: center;
        justify-content: space-between;
        flex-direction: column;
    } 
    .right button{
        background: transparent;
        border: none;
    }  
</style>