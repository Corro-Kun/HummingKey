<script>
    import {onMount} from 'svelte';
    import Trash from '@/components/icons/Trash.svelte';
    import Pencil from '@/components/icons/Pencil.svelte';
    import AccountIcons from '@/components/AccountIcons.svelte';
    import Copy from '@/components/icons/Copy.svelte';
    import Eye from '@/components/icons/Eye.svelte';

    let data = [];

    let index = null;

    onMount(async ()=>{
        const { invoke } = await import('@tauri-apps/api');
        data = await invoke("get_passwords");
    });
</script>

<div class="card" >
    <div class="list" >
        <div class="title" >
            <h2>Contraseñas</h2>
        </div>
        <div class="list-div" >
            {#each data as item, i}
                <div on:click={()=> index=i} >
                    {#if index === i}
                    <p>-</p>
                    {/if}
                    <AccountIcons icon={item.icon} />
                    <p class="text-pass" >{item.name}</p>
                </div>
            {/each}
        </div>
    </div>
    {#if index !== null}
    <div class="content" >
        <div class="left" >
            <div class="title-content" >
                <AccountIcons icon={data[index].icon} />
                <h2>{data[index].name}</h2>
            </div>
            <div class="passwords" >
                <div class="pass-content" >
                    <div>
                        <h3>Email</h3>
                        <p>{data[index].user}</p>
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
                        <p>{data[index].password}</p>
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
    {/if}
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
        margin-bottom: 10px;
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