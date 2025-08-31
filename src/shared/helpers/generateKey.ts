export const generateKey = (prefix: string, index: number): string => {
  return `${prefix}_${index}`;
};