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

    export let icon;
</script>

{#if icon === 1}
<Box />
{:else if icon === 2}
<Book />
{:else if icon === 3}
<Store />
{:else if icon === 4}
<Work />
{:else if icon === 5}
<Game />
{:else if icon === 6}
<Cloudy />
{:else if icon === 7}
<Google />
{:else if icon === 8}
<Windows />
{:else if icon === 9}
<Github />
{:else if icon === 10}
<Facebook />
{:else if icon === 11}
<X />
{:else if icon === 12}
<Reddit />
{:else if icon === 13}
<Spotify />
{:else if icon === 14}
<Netflix />
{:else if icon === 15}
<Discord />
{:else if icon === 16}
<Steam />
{/if}
