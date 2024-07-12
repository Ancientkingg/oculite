<script setup>
import { ref, onBeforeMount, reactive } from 'vue';
import { FilterMatchMode, FilterOperator } from 'primevue/api';
import { CustomerService } from '@/service/CustomerService';
import { ProductService } from '@/service/ProductService';

const customer1 = ref(null);
const customer2 = ref(null);
const customer3 = ref(null);
const filters1 = ref(null);
const loading1 = ref(null);
const loading2 = ref(null);
const idFrozen = ref(false);
const products = ref(null);
const expandedRows = ref([]);
const statuses = reactive(['unqualified', 'qualified', 'new', 'negotiation', 'renewal', 'proposal']);
const representatives = reactive([
    { name: 'Amy Elsner', image: 'amyelsner.png' },
    { name: 'Anna Fali', image: 'annafali.png' },
    { name: 'Asiya Javayant', image: 'asiyajavayant.png' },
    { name: 'Bernardo Dominic', image: 'bernardodominic.png' },
    { name: 'Elwin Sharvill', image: 'elwinsharvill.png' },
    { name: 'Ioni Bowcher', image: 'ionibowcher.png' },
    { name: 'Ivan Magalhaes', image: 'ivanmagalhaes.png' },
    { name: 'Onyama Limba', image: 'onyamalimba.png' },
    { name: 'Stephen Shaw', image: 'stephenshaw.png' },
    { name: 'XuXue Feng', image: 'xuxuefeng.png' }
]);

const customerService = new CustomerService();
const productService = new ProductService();

const getBadgeSeverity = (inventoryStatus) => {
    switch (inventoryStatus.toLowerCase()) {
        case 'instock':
            return 'success';
        case 'lowstock':
            return 'warning';
        case 'outofstock':
            return 'danger';
        default:
            return 'info';
    }
};
const getSeverity = (status) => {
    switch (status) {
        case 'unqualified':
            return 'danger';

        case 'qualified':
            return 'success';

        case 'new':
            return 'info';

        case 'negotiation':
            return 'warning';

        case 'renewal':
            return null;
    }
};

onBeforeMount(() => {
    productService.getProductsWithOrdersSmall().then((data) => (products.value = data));
    customerService.getCustomersLarge().then((data) => {
        customer1.value = data;
        loading1.value = false;
        customer1.value.forEach((customer) => (customer.date = new Date(customer.date)));
    });
    customerService.getCustomersLarge().then((data) => (customer2.value = data));
    customerService.getCustomersMedium().then((data) => (customer3.value = data));
    loading2.value = false;

    initFilters1();
});

const initFilters1 = () => {
    filters1.value = {
        global: { value: null, matchMode: FilterMatchMode.CONTAINS },
        name: { operator: FilterOperator.AND, constraints: [{ value: null, matchMode: FilterMatchMode.STARTS_WITH }] },
        'country.name': { operator: FilterOperator.AND, constraints: [{ value: null, matchMode: FilterMatchMode.STARTS_WITH }] },
        representative: { value: null, matchMode: FilterMatchMode.IN },
        date: { operator: FilterOperator.AND, constraints: [{ value: null, matchMode: FilterMatchMode.DATE_IS }] },
        balance: { operator: FilterOperator.AND, constraints: [{ value: null, matchMode: FilterMatchMode.EQUALS }] },
        status: { operator: FilterOperator.OR, constraints: [{ value: null, matchMode: FilterMatchMode.EQUALS }] },
        activity: { value: [0, 100], matchMode: FilterMatchMode.BETWEEN },
        verified: { value: null, matchMode: FilterMatchMode.EQUALS }
    };
};

const clearFilter1 = () => {
    initFilters1();
};
const expandAll = () => {
    expandedRows.value = products.value.reduce((acc, p) => (acc[p.id] = true) && acc, {});
};
const collapseAll = () => {
    expandedRows.value = null;
};
const formatCurrency = (value) => {
    return value.toLocaleString('en-US', { style: 'currency', currency: 'USD' });
};

const formatDate = (value) => {
    return value.toLocaleDateString('en-US', {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric'
    });
};
const calculateCustomerTotal = (name) => {
    let total = 0;
    if (customer3.value) {
        for (let customer of customer3.value) {
            if (customer.representative.name === name) {
                total++;
            }
        }
    }

    return total;
};
</script>

<template>
    <div class="grid">
        <div class="col-12">
            <div class="card">
                <h5>Filter Menu</h5>
                <DataTable
                    v-model:filters="filters1"
                    :value="customer1"
                    :paginator="true"
                    :rows="10"
                    data-key="id"
                    :row-hover="true"
                    filter-display="menu"
                    :loading="loading1"
                    :filters="filters1"
                    :global-filter-fields="['name', 'country.name', 'representative.name', 'balance', 'status']"
                    show-gridlines
                    >
                    <template #header>
                        <div class="flex justify-content-between flex-column sm:flex-row">
                            <Button
                                type="button"
                                icon="pi pi-filter-slash"
                                label="Clear"
                                outlined
                                @click="clearFilter1()"
                                />
                            <IconField icon-position="left">
                                <InputIcon class="pi pi-search" />
                                <InputText
                                    v-model="filters1['global'].value"
                                    placeholder="Keyword Search"
                                    style="width: 100%"
                                    />
                            </IconField>
                        </div>
                    </template>
                    <template #empty>
                        No customers found.
                    </template>
                    <template #loading>
                        Loading customers data. Please wait.
                    </template>
                    <Column
                        field="name"
                        header="Name"
                        style="min-width: 12rem"
                        >
                        <template #body="{ data }">
                            {{ data.name }}
                        </template>
                        <template #filter="{ filterModel }">
                            <InputText
                                v-model="filterModel.value"
                                type="text"
                                class="p-column-filter"
                                placeholder="Search by name"
                                />
                        </template>
                    </Column>
                    <Column
                        header="Country"
                        filter-field="country.name"
                        style="min-width: 12rem"
                        >
                        <template #body="{ data }">
                            <div class="flex align-items-center gap-2">
                                <img
                                    alt="flag"
                                    src="/demo/images/flag/flag_placeholder.png"
                                    :class="`flag flag-${data.country.code}`"
                                    style="width: 24px"
                                    >
                                <span>{{ data.country.name }}</span>
                            </div>
                        </template>
                        <template #filter="{ filterModel }">
                            <InputText
                                v-model="filterModel.value"
                                type="text"
                                class="p-column-filter"
                                placeholder="Search by country"
                                />
                        </template>
                        <template #filterclear="{ filterCallback }">
                            <Button
                                type="button"
                                icon="pi pi-times"
                                severity="secondary"
                                @click="filterCallback()"
                                />
                        </template>
                        <template #filterapply="{ filterCallback }">
                            <Button
                                type="button"
                                icon="pi pi-check"
                                severity="success"
                                @click="filterCallback()"
                                />
                        </template>
                    </Column>
                    <Column
                        header="Agent"
                        filter-field="representative"
                        :show-filter-match-modes="false"
                        :filter-menu-style="{ width: '14rem' }"
                        style="min-width: 14rem"
                        >
                        <template #body="{ data }">
                            <div class="flex align-items-center gap-2">
                                <img
                                    :alt="data.representative.name"
                                    :src="`https://primefaces.org/cdn/primevue/images/avatar/${data.representative.image}`"
                                    style="width: 32px"
                                    >
                                <span>{{ data.representative.name }}</span>
                            </div>
                        </template>
                        <template #filter="{ filterModel }">
                            <div class="mb-3 text-bold">
                                Agent Picker
                            </div>
                            <MultiSelect
                                v-model="filterModel.value"
                                :options="representatives"
                                option-label="name"
                                placeholder="Any"
                                class="p-column-filter"
                                >
                                <template #option="slotProps">
                                    <div class="p-multiselect-representative-option">
                                        <img
                                            :alt="slotProps.option.name"
                                            :src="'/demo/images/avatar/' + slotProps.option.image"
                                            width="32"
                                            style="vertical-align: middle"
                                            >
                                        <span
                                            style="margin-left: 0.5em; vertical-align: middle"
                                            class="image-text"
                                            >{{ slotProps.option.name }}</span>
                                    </div>
                                </template>
                            </MultiSelect>
                        </template>
                    </Column>
                    <Column
                        header="Date"
                        filter-field="date"
                        data-type="date"
                        style="min-width: 10rem"
                        >
                        <template #body="{ data }">
                            {{ formatDate(data.date) }}
                        </template>
                        <template #filter="{ filterModel }">
                            <Calendar
                                v-model="filterModel.value"
                                date-format="mm/dd/yy"
                                placeholder="mm/dd/yyyy"
                                />
                        </template>
                    </Column>
                    <Column
                        header="Balance"
                        filter-field="balance"
                        data-type="numeric"
                        style="min-width: 10rem"
                        >
                        <template #body="{ data }">
                            {{ formatCurrency(data.balance) }}
                        </template>
                        <template #filter="{ filterModel }">
                            <InputNumber
                                v-model="filterModel.value"
                                mode="currency"
                                currency="USD"
                                locale="en-US"
                                />
                        </template>
                    </Column>
                    <Column
                        field="status"
                        header="Status"
                        :filter-menu-style="{ width: '14rem' }"
                        style="min-width: 12rem"
                        >
                        <template #body="{ data }">
                            <Tag :severity="getSeverity(data.status)">
                                {{ data.status.toUpperCase() }}
                            </Tag>
                        </template>
                        <template #filter="{ filterModel }">
                            <Dropdown
                                v-model="filterModel.value"
                                :options="statuses"
                                placeholder="Any"
                                class="p-column-filter"
                                :show-clear="true"
                                >
                                <template #value="slotProps">
                                    <Tag
                                        v-if="slotProps.value"
                                        :severity="getSeverity(slotProps.value)"
                                        >
                                        {{ slotProps.value }}
                                    </Tag>
                                    <span v-else>{{ slotProps.placeholder }}</span>
                                </template>
                                <template #option="slotProps">
                                    <Tag :severity="getSeverity(slotProps.option)">
                                        {{ slotProps.option.toUpperCase() }}
                                    </Tag>
                                </template>
                            </Dropdown>
                        </template>
                    </Column>
                    <Column
                        field="activity"
                        header="Activity"
                        :show-filter-match-modes="false"
                        style="min-width: 12rem"
                        >
                        <template #body="{ data }">
                            <ProgressBar
                                :value="data.activity"
                                :show-value="false"
                                style="height: 0.5rem"
                                />
                        </template>
                        <template #filter="{ filterModel }">
                            <Slider
                                v-model="filterModel.value"
                                :range="true"
                                class="m-3"
                                />
                            <div class="flex align-items-center justify-content-between px-2">
                                <span>{{ filterModel.value ? filterModel.value[0] : 0 }}</span>
                                <span>{{ filterModel.value ? filterModel.value[1] : 100 }}</span>
                            </div>
                        </template>
                    </Column>
                    <Column
                        field="verified"
                        header="Verified"
                        data-type="boolean"
                        body-class="text-center"
                        style="min-width: 8rem"
                        >
                        <template #body="{ data }">
                            <i
                                class="pi"
                                :class="{ 'text-green-500 pi-check-circle': data.verified, 'text-pink-500 pi-times-circle': !data.verified }"
                                />
                        </template>
                        <template #filter="{ filterModel }">
                            <TriStateCheckbox v-model="filterModel.value" />
                        </template>
                    </Column>
                </DataTable>
            </div>
        </div>

        <div class="col-12">
            <div class="card">
                <h5>Frozen Columns</h5>
                <ToggleButton
                    v-model="idFrozen"
                    on-icon="pi pi-lock"
                    off-icon="pi pi-lock-open"
                    on-label="Unfreeze Id"
                    off-label="Freeze Id"
                    style="width: 10rem"
                    />

                <DataTable
                    :value="customer2"
                    :scrollable="true"
                    scroll-height="400px"
                    :loading="loading2"
                    scroll-direction="both"
                    class="mt-3"
                    >
                    <Column
                        field="name"
                        header="Name"
                        style="min-width: 200px"
                        frozen
                        />
                    <Column
                        field="id"
                        header="Id"
                        style="min-width: 100px"
                        :frozen="idFrozen"
                        />
                    <Column
                        field="country.name"
                        header="Country"
                        :style="{ width: '200px' }"
                        >
                        <template #body="{ data }">
                            <div class="flex align-items-center gap-2">
                                <img
                                    alt="flag"
                                    src="/demo/images/flag/flag_placeholder.png"
                                    :class="`flag flag-${data.country.code}`"
                                    style="width: 24px"
                                    >
                                <span>{{ data.country.name }}</span>
                            </div>
                        </template>
                    </Column>
                    <Column
                        field="date"
                        header="Date"
                        style="min-width: 200px"
                        />
                    <Column
                        field="company"
                        header="Company"
                        style="min-width: 200px"
                        />
                    <Column
                        field="status"
                        header="Status"
                        style="min-width: 200px"
                        >
                        <template #body="{ data }">
                            <Tag :severity="getSeverity(data.status)">
                                {{ data.status.toUpperCase() }}
                            </Tag>
                        </template>
                    </Column>
                    <Column
                        field="activity"
                        header="Activity"
                        style="min-width: 200px"
                        />
                    <Column
                        field="representative.name"
                        header="Representative"
                        style="min-width: 200px"
                        >
                        <template #body="{ data }">
                            <div class="flex align-items-center gap-2">
                                <img
                                    :alt="data.representative.name"
                                    :src="`https://primefaces.org/cdn/primevue/images/avatar/${data.representative.image}`"
                                    style="width: 32px"
                                    >
                                <span>{{ data.representative.name }}</span>
                            </div>
                        </template>
                    </Column>
                    <Column
                        field="balance"
                        header="Balance"
                        style="min-width: 200px"
                        frozen
                        align-frozen="right"
                        >
                        <template #body="{ data }">
                            <span class="text-bold">{{ formatCurrency(data.balance) }}</span>
                        </template>
                    </Column>
                </DataTable>
            </div>
        </div>

        <div class="col-12">
            <div class="card">
                <h5>Row Expand</h5>
                <DataTable
                    v-model:expandedRows="expandedRows"
                    :value="products"
                    data-key="id"
                    table-style="min-width: 60rem"
                    >
                    <template #header>
                        <div>
                            <Button
                                icon="pi pi-plus"
                                label="Expand All"
                                class="mr-2 mb-2"
                                @click="expandAll"
                                />
                            <Button
                                icon="pi pi-minus"
                                label="Collapse All"
                                class="mb-2"
                                @click="collapseAll"
                                />
                        </div>
                    </template>
                    <Column
                        :expander="true"
                        header-style="width: 3rem"
                        />
                    <Column
                        field="name"
                        header="Name"
                        :sortable="true"
                        >
                        <template #body="slotProps">
                            {{ slotProps.data.name }}
                        </template>
                    </Column>
                    <Column header="Image">
                        <template #body="slotProps">
                            <img
                                :src="'/demo/images/product/' + slotProps.data.image"
                                :alt="slotProps.data.image"
                                class="shadow-2"
                                width="100"
                                >
                        </template>
                    </Column>
                    <Column
                        field="price"
                        header="Price"
                        :sortable="true"
                        >
                        <template #body="slotProps">
                            {{ formatCurrency(slotProps.data.price) }}
                        </template>
                    </Column>
                    <Column
                        field="category"
                        header="Category"
                        :sortable="true"
                        >
                        <template #body="slotProps">
                            {{ formatCurrency(slotProps.data.category) }}
                        </template>
                    </Column>
                    <Column
                        field="rating"
                        header="Reviews"
                        :sortable="true"
                        >
                        <template #body="slotProps">
                            <Rating
                                :model-value="slotProps.data.rating"
                                :readonly="true"
                                :cancel="false"
                                />
                        </template>
                    </Column>
                    <Column
                        field="inventoryStatus"
                        header="Status"
                        :sortable="true"
                        >
                        <template #body="slotProps">
                            <Tag :severity="getBadgeSeverity(slotProps.data.inventoryStatus)">
                                {{ slotProps.data.inventoryStatus }}
                            </Tag>
                        </template>
                    </Column>
                    <template #expansion="slotProps">
                        <div class="p-3">
                            <h5>Orders for {{ slotProps.data.name }}</h5>
                            <DataTable :value="slotProps.data.orders">
                                <Column
                                    field="id"
                                    header="Id"
                                    :sortable="true"
                                    >
                                    <template #body="slotProps">
                                        {{ slotProps.data.id }}
                                    </template>
                                </Column>
                                <Column
                                    field="customer"
                                    header="Customer"
                                    :sortable="true"
                                    >
                                    <template #body="slotProps">
                                        {{ slotProps.data.customer }}
                                    </template>
                                </Column>
                                <Column
                                    field="date"
                                    header="Date"
                                    :sortable="true"
                                    >
                                    <template #body="slotProps">
                                        {{ slotProps.data.date }}
                                    </template>
                                </Column>
                                <Column
                                    field="amount"
                                    header="Amount"
                                    :sortable="true"
                                    >
                                    <template #body="slotProps">
                                        {{ formatCurrency(slotProps.data.amount) }}
                                    </template>
                                </Column>
                                <Column
                                    field="status"
                                    header="Status"
                                    :sortable="true"
                                    >
                                    <template #body="slotProps">
                                        <span :class="'order-badge order-' + (slotProps.data.status ? slotProps.data.status.toLowerCase() : '')">{{ slotProps.data.status }}</span>
                                    </template>
                                </Column>
                                <Column header-style="width:4rem">
                                    <template #body>
                                        <Button icon="pi pi-search" />
                                    </template>
                                </Column>
                            </DataTable>
                        </div>
                    </template>
                </DataTable>
            </div>
        </div>

        <div class="col-12">
            <div class="card">
                <h5>Subheader Grouping</h5>
                <DataTable
                    :value="customer3"
                    row-group-mode="subheader"
                    group-rows-by="representative.name"
                    sort-mode="single"
                    sort-field="representative.name"
                    :sort-order="1"
                    scrollable
                    scroll-height="400px"
                    >
                    <Column
                        field="representative.name"
                        header="Representative"
                        />
                    <Column
                        field="name"
                        header="Name"
                        style="min-width: 200px"
                        />
                    <Column
                        field="country"
                        header="Country"
                        style="min-width: 200px"
                        >
                        <template #body="{ data }">
                            <div class="flex align-items-center gap-2">
                                <img
                                    alt="flag"
                                    src="/demo/images/flag/flag_placeholder.png"
                                    :class="`flag flag-${data.country.code}`"
                                    style="width: 24px"
                                    >
                                <span>{{ data.country.name }}</span>
                            </div>
                        </template>
                    </Column>
                    <Column
                        field="company"
                        header="Company"
                        style="min-width: 200px"
                        />
                    <Column
                        field="status"
                        header="Status"
                        style="min-width: 200px"
                        >
                        <template #body="slotProps">
                            <Tag :severity="getSeverity(slotProps.data.status)">
                                {{ slotProps.data.status.toUpperCase() }}
                            </Tag>
                        </template>
                    </Column>
                    <Column
                        field="date"
                        header="Date"
                        style="min-width: 200px"
                        />
                    <template #groupheader="slotProps">
                        <div class="flex align-items-center gap-2">
                            <img
                                :alt="slotProps.data.representative.name"
                                :src="'/demo/images/avatar/' + slotProps.data.representative.image"
                                width="32"
                                style="vertical-align: middle"
                                >
                            <span>{{ slotProps.data.representative.name }}</span>
                        </div>
                    </template>
                    <template #groupfooter="slotProps">
                        <td
                            style="text-align: right"
                            class="text-bold pr-6"
                            >
                            Total Customers: {{ calculateCustomerTotal(slotProps.data.representative.name) }}
                        </td>
                    </template>
                </DataTable>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
:deep(.p-datatable-frozen-tbody) {
    font-weight: bold;
}

:deep(.p-datatable-scrollable .p-frozen-column) {
    font-weight: bold;
}
</style>
