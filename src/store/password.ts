import {writable} from 'svelte/store';

export const request = writable(false);
export const newPassword = writable({
    id: 0,
    name: "",
    icon: 0,
    user: "",
    password: "",
    password_length: 0,
});