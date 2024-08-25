<script>
    import { onMount } from 'svelte';
    import {navigate} from 'astro:transitions/client';

    onMount(async () => {
        const { invoke } = await import('@tauri-apps/api');
        getResult(invoke);
    });

    async function getResult(invoke){
        let result = await invoke("verify_db");
        if (result){
            navigate("/mainLogin");
        } else {
            navigate("/newUser");
        }
    }
</script>