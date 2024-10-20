<script>
    import {onMount} from 'svelte';
    import {profileImg} from '@/store/profile.ts'

    onMount(async ()=>{
		const { invoke } = await import('@tauri-apps/api');
		const img = await invoke("get_image_user");
        profileImg.set(img);
	});
</script>

<picture out:fade|keepUpdatingState >
    <img src={$profileImg} alt="profile" loading="lazy" >
</picture>

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