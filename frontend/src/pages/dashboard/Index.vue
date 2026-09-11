<script setup lang="ts">
import { ArrowUpRight, ArrowDownRight, DollarSign, Users, Receipt } from 'lucide-vue-next'
</script>

<template>
  <div class="max-w-7xl mx-auto space-y-6">
    <div class="flex justify-between items-center">
      <h1 class="text-2xl font-semibold text-gray-900">Dashboard Overview</h1>
    </div>

    <!-- Metrics Cards -->
    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
      <div class="bg-white overflow-hidden rounded-lg shadow-sm border border-gray-100">
        <div class="p-5">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <DollarSign class="h-6 w-6 text-gray-400" />
            </div>
            <div class="ml-5 w-0 flex-1">
              <dl>
                <dt class="text-sm font-medium text-gray-500 truncate">Total Revenue</dt>
                <dd>
                  <div class="text-lg font-medium text-gray-900">Rp 120,500,000</div>
                </dd>
              </dl>
            </div>
          </div>
        </div>
        <div class="bg-gray-50 px-5 py-3 border-t border-gray-100">
          <div class="text-sm">
            <div class="flex items-center text-emerald-600">
              <ArrowUpRight class="w-4 h-4 mr-1" />
              <span>12% from last month</span>
            </div>
          </div>
        </div>
      </div>

      <div class="bg-white overflow-hidden rounded-lg shadow-sm border border-gray-100">
        <div class="p-5">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <DollarSign class="h-6 w-6 text-gray-400" />
            </div>
            <div class="ml-5 w-0 flex-1">
              <dl>
                <dt class="text-sm font-medium text-gray-500 truncate">Total Expenses</dt>
                <dd>
                  <div class="text-lg font-medium text-gray-900">Rp 45,200,000</div>
                </dd>
              </dl>
            </div>
          </div>
        </div>
        <div class="bg-gray-50 px-5 py-3 border-t border-gray-100">
          <div class="text-sm">
            <div class="flex items-center text-rose-600">
              <ArrowDownRight class="w-4 h-4 mr-1" />
              <span>5% from last month</span>
            </div>
          </div>
        </div>
      </div>

      <div class="bg-white overflow-hidden rounded-lg shadow-sm border border-gray-100">
        <div class="p-5">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <Users class="h-6 w-6 text-gray-400" />
            </div>
            <div class="ml-5 w-0 flex-1">
              <dl>
                <dt class="text-sm font-medium text-gray-500 truncate">Active Customers</dt>
                <dd>
                  <div class="text-lg font-medium text-gray-900">142</div>
                </dd>
              </dl>
            </div>
          </div>
        </div>
        <div class="bg-gray-50 px-5 py-3 border-t border-gray-100">
           <div class="text-sm">
            <div class="flex items-center text-emerald-600">
              <ArrowUpRight class="w-4 h-4 mr-1" />
              <span>8 new this week</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Activity Placeholder -->
    <div class="bg-white shadow-sm rounded-lg border border-gray-100">
      <div class="px-4 py-5 sm:px-6 border-b border-gray-100">
        <h3 class="text-lg leading-6 font-medium text-gray-900">Recent Transactions</h3>
      </div>
      <div class="p-12 flex flex-col justify-center items-center h-48 text-gray-500">
        <Receipt class="h-10 w-10 text-gray-400 mb-3" />
        <h4 class="text-sm font-medium text-gray-900">No recent transactions</h4>
        <p class="text-sm text-gray-500 mt-1">When you record transactions, they will appear here.</p>
      </div>
    </div>
  </div>
</template>
