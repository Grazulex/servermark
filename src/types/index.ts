// Service types
export type ServiceStatus = 'running' | 'stopped' | 'starting' | 'stopping' | 'error'

export interface Service {
  id: string
  name: string
  status: ServiceStatus
  port: number
  pid?: number
  error?: string
}

// Site types
export interface Site {
  id: string
  name: string
  path: string
  phpVersion: string
  secured: boolean
  domain: string
  type: 'laravel' | 'symfony' | 'wordpress' | 'static' | 'proxy'
  proxyTarget?: string
}

// PHP types
export interface PhpVersion {
  version: string
  installed: boolean
  active: boolean
  path: string
  extensions?: string[]
}

// Config types
export type LinuxDistro =
  | 'ubuntu'
  | 'debian'
  | 'fedora'
  | 'arch'
  | 'opensuse'
  | 'manjaro'
  | 'mint'
  | 'pop'
  | 'elementary'
  | 'zorin'
  | 'unknown'

export type PackageManager = 'apt' | 'dnf' | 'pacman' | 'zypper'

export type WebServer = 'nginx' | 'caddy'

export interface Config {
  tld: string
  loopback: string
  sitesPath: string
  webServer: WebServer
  distro: LinuxDistro
  packageManager: PackageManager
}

// System info
export interface SystemInfo {
  distro: LinuxDistro
  distroVersion: string
  packageManager: PackageManager
  kernel: string
  hostname: string
}
