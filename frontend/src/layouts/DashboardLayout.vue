<script setup lang="ts">
import { RouterView, RouterLink } from 'vue-router'
import { LayoutDashboard, Receipt, FileText, Settings, Users, LogOut } from 'lucide-vue-next'
</script>

<template>
  <div class="flex h-screen bg-gray-50">
    <!-- Sidebar -->
    <aside class="w-64 bg-white border-r border-gray-200 hidden md:flex flex-col">
      <div class="h-16 flex items-center px-6 border-b border-gray-200">
        <h1 class="text-xl font-bold text-indigo-600">DUITGOO</h1>
      </div>
      <nav class="flex-1 overflow-y-auto py-4">
        <ul class="space-y-1 px-3">
          <li>
            <RouterLink to="/dashboard" class="flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100 hover:text-gray-900" active-class="bg-indigo-50 text-indigo-700 font-medium">
              <LayoutDashboard class="w-5 h-5" />
              Dashboard
            </RouterLink>
          </li>
          <li>
            <RouterLink to="/sales" class="flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100 hover:text-gray-900" active-class="bg-indigo-50 text-indigo-700 font-medium">
              <Receipt class="w-5 h-5" />
              Sales
            </RouterLink>
          </li>
          <li>
            <RouterLink to="/reports" class="flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100 hover:text-gray-900" active-class="bg-indigo-50 text-indigo-700 font-medium">
              <FileText class="w-5 h-5" />
              Reports
            </RouterLink>
          </li>
          <li>
            <RouterLink to="/customers" class="flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100 hover:text-gray-900" active-class="bg-indigo-50 text-indigo-700 font-medium">
              <Users class="w-5 h-5" />
              Customers
            </RouterLink>
          </li>
          <li>
            <RouterLink to="/settings" class="flex items-center gap-3 px-3 py-2 rounded-md text-gray-700 hover:bg-gray-100 hover:text-gray-900" active-class="bg-indigo-50 text-indigo-700 font-medium">
              <Settings class="w-5 h-5" />
              Settings
            </RouterLink>
          </li>
        </ul>
      </nav>
      <div class="p-4 border-t border-gray-200">
        <button class="flex items-center gap-3 w-full px-3 py-2 text-gray-700 hover:bg-gray-100 rounded-md transition-colors">
          <LogOut class="w-5 h-5 text-gray-400" />
          <span class="text-sm font-medium">Sign Out</span>
        </button>
      </div>
    </aside>

    <!-- Main Content -->
    <div class="flex-1 flex flex-col min-w-0 overflow-hidden">
      <!-- Header -->
      <header class="h-16 bg-white border-b border-gray-200 flex items-center justify-between px-4 sm:px-6 lg:px-8">
        <div class="flex items-center md:hidden">
          <h1 class="text-xl font-bold text-indigo-600">DUITGOO</h1>
        </div>
        <div class="flex-1 flex justify-end">
          <div class="ml-4 flex items-center md:ml-6">
            <div class="flex items-center gap-2">
              <div class="w-8 h-8 rounded-full bg-indigo-100 flex items-center justify-center text-indigo-700 font-bold">
                JD
              </div>
              <span class="text-sm font-medium text-gray-700 hidden sm:block">John Doe</span>
            </div>
          </div>
        </div>
      </header>

      <!-- Main scrollable area -->
      <main class="flex-1 overflow-y-auto bg-gray-50 p-4 sm:p-6 lg:p-8">
        <RouterView />
      </main>
    </div>
  </div>
</template>
