# Chur

*"chur" - New Zealand Slang for Cheers!*

Chur is wrapper around `tonic-build` that makes it simpler to declare dependencies on other types.

At the moment it only supports github repos which it will download as tarballs.

> [!NOTE]
> This uses some fairly non-standard methods of caching stuff
>
> i.e. it creates a `chur` folder in the `target` dir, meaning it should only be invoked once per workspace at risk of overwriting.
>
> I have no idea if it's ACTUALLY secure, if you have any better ideas or suggestions, please open an issue.

there's an example in the [example](./example) folder.

will update with better docs eventually.