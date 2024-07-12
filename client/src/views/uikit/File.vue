<script setup>
import { useToast } from 'primevue/usetoast';

const toast = useToast();

const onUpload = () => {
    toast.add({ severity: 'info', summary: 'Success', detail: 'File Uploaded', life: 3000 });
};
</script>

<template>
    <div class="grid">
        <div class="col-12">
            <div class="card">
                <h5>Advanced</h5>
                <FileUpload
                    name="demo[]"
                    :multiple="true"
                    accept="image/*"
                    :max-file-size="1000000"
                    custom-upload
                    @uploader="onUpload"
                    />

                <h5>Basic</h5>
                <FileUpload
                    mode="basic"
                    name="demo[]"
                    accept="image/*"
                    :max-file-size="1000000"
                    custom-upload
                    @uploader="onUpload"
                    />
            </div>
        </div>
    </div>
</template>
