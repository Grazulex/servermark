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
  php_version: string
  secured: boolean
  domain: string
  site_type: 'laravel' | 'symfony' | 'wordpress' | 'static' | 'proxy'
  proxy_target?: string
  laravel?: LaravelInfo
}

// Laravel types
export interface LaravelInfo {
  detected: boolean
  version: string | null
  constraint: string | null
  php_version: string | null
  has_update?: boolean
  latest_version?: string | null
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

export interface Config {
  tld: string
  loopback: string
  sitesPath: string
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
