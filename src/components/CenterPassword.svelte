<script>
    import Trash from '@/components/icons/Trash.svelte';
    import Pencil from '@/components/icons/Pencil.svelte';
</script>

<div class="card" >
    <div class="list" >
        <div class="title" >
            <h3>Contraseñas</h3>
        </div>
        <div class="list-div" >
            <h2>hola</h2>
            <h2>hola</h2>
            <h2>hola</h2>
            <h2>hola</h2>
            <h2>hola</h2>
            <h2>hola</h2>
            <h2>hola</h2>
            <h2>hola</h2>
            <h2>hola</h2>
            <h2>hola</h2>
            <h2>hola</h2>
            <h2>hola</h2>
        </div>
    </div>
    <div class="content" >
        <div class="left" >

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
    .title h3{
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
    .content{
        display: flex;
        height: 250px;
        width: 400px;
        background: transparent;
        backdrop-filter: blur(10px);
        border: 2px solid var(--Color_Primary);
        border-radius: 8px;
        overflow: hidden;
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