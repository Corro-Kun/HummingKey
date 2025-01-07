<script>
    import { onMount } from 'svelte';
    import {navigate} from 'astro:transitions/client';
    import { check } from '@tauri-apps/plugin-updater';

    onMount(async () => {
        const { invoke } = await import('@tauri-apps/api/core');
        getResult(invoke);
    });

    async function getResult(invoke){
        try {
            const update = await check();
            if (update?.available) {
                return navigate("/updater");
            } 
        } catch (error) {
            console.error(error);
        }
       
        let result = await invoke("verify_db");
        if (result){
            navigate("/mainLogin");
        } else {
            navigate("/newUser");
        }
    }
</script>