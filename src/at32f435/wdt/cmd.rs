#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Command register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CMD_A {
    #[doc = "21845: Unlock write-protected WDT_DIV and WDT_RLD"]
    Unlock = 21845,
    #[doc = "43690: Reload counter"]
    Reload = 43690,
    #[doc = "52428: Enable WDT. If the hardware watchdog has been enabled, ignore this operation."]
    Enable = 52428,
}
impl From<CMD_A> for u16 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMD_A {
    type Ux = u16;
}
impl crate::IsEnum for CMD_A {}
#[doc = "Field `CMD` writer - Command register"]
pub type CMD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, CMD_A>;
impl<'a, REG> CMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Unlock write-protected WDT_DIV and WDT_RLD"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::Unlock)
    }
    #[doc = "Reload counter"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::Reload)
    }
    #[doc = "Enable WDT. If the hardware watchdog has been enabled, ignore this operation."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_A::Enable)
    }
}
impl core::fmt::Debug for crate::generic::Reg<CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Command register"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<CMD_SPEC> {
        CMD_W::new(self, 0)
    }
}
#[doc = "Command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
