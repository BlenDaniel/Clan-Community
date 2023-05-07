export default interface User {
    id?: any | null,
    username?: string | null,
    email?: string,
    password?: string,
    roles?: Array<string>
  }