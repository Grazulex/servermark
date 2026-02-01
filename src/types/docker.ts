// Docker/Podman types

export type ContainerRuntime = 'docker' | 'podman' | 'none'

export type ContainerStatus = 'running' | 'stopped' | 'paused' | 'restarting' | 'removing' | 'created'

export interface Container {
  id: string
  name: string
  image: string
  status: ContainerStatus | string
  ports: PortMapping[]
  created: string
  service?: ServiceType
}

export interface PortMapping {
  host: number
  container: number
  protocol: 'tcp' | 'udp'
}

export type ServiceType =
  | 'mysql'
  | 'postgresql'
  | 'redis'
  | 'mailpit'
  | 'minio'
  | 'adminer'
  | 'phpmyadmin'
  | 'memcached'
  | 'mongodb'
  | 'custom'

export interface ServiceTemplate {
  id: ServiceType
  name: string
  description: string
  image: string
  defaultTag: string
  availableTags: string[]
  ports: PortMapping[]
  environment: Record<string, string>
  volumes?: VolumeMapping[]
  category: 'database' | 'cache' | 'mail' | 'storage' | 'tools'
}

export interface VolumeMapping {
  name: string
  container: string
  description: string
}

export interface RuntimeInfo {
  runtime: ContainerRuntime | string
  version: string
  apiVersion?: string
  api_version?: string
  available: boolean
}
