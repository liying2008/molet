export enum ContentType {
  Unicode = 'Unicode',
  Bitmap = 'Bitmap',
  FileList = 'FileList',
  RawData = 'RawData',
}

export class StagingData {
  id: number | null = null
  contentType: ContentType = ContentType.RawData
  creationTime: number = 0
  title: string = ''
  content: Array<number> = []
}
