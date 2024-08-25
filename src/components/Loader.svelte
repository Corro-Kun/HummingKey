<script>
    export let width = 24;
    export let height = 24;
</script>

<svg  xmlns="http://www.w3.org/2000/svg"  width={width}  height={height}  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-loader-2">
    <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
    <path d="M12 3a9 9 0 1 0 9 9" />
</svg>

<style>
    svg{
        color: var(--Color_Text);
    }
    .icon{
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
</style>