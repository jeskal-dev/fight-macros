export interface IDataConvertible<TData> {
  toData(): TData;
}

export interface IFormConvertible<TForm> {
  toForm(): TForm;
}