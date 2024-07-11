/** Helper for throwing errors in expression positions */
export const raise = (error: unknown): never => {
  throw typeof error === "string" ? new Error(error) : error;
};
