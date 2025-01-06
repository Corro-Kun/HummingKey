<script>
    import {onMount} from 'svelte';
    import {profileImg} from '@/store/profile.ts'
    import {navigate} from 'astro:transitions/client';
    
    let mounted = $state(false);

    onMount(async ()=>{
        const { invoke } = await import('@tauri-apps/api/core');
        const img = await invoke("get_image_user");
        profileImg.set(img);
        mounted = true;
    });
</script>

{#if mounted && $profileImg}
    <picture out:fade|keepUpdatingState onclick={()=> navigate("/user")} >
        <img src={$profileImg} alt="profile" loading="lazy">
    </picture>
{/if}

<style>
    picture{
        display: flex;
        height: 45px;
        width: 45px;
        overflow: hidden;
        border-radius: 50%;
        align-items: center;
        justify-content: center;
        border: 2px solid var(--Color_Primary);
        transition: .2s;
        cursor: pointer;
    }
    picture:hover{
        border: 2px solid var(--Color_Secondary);
    }
    img{
        height: 50px;
    }
</style>