<svg  xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-loader-quarter">
    <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
    <path d="M12 6l0 -3" />
    <path d="M6 12l-3 0" />
    <path d="M7.75 7.75l-2.15 -2.15" />
</svg>

<style>
    .loader {
    width: 100px;
    height: 100px;
    border-radius: 50%;
    background: conic-gradient(#3498db 0% 0%, #e0e0e0 0% 100%);
    animation: spin 5s linear infinite;
    }

  .icon {
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