#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
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
#[doc = "Field `CMD` reader - Command register"]
pub type CMD_R = crate::FieldReader<CMD_A>;
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            21845 => Some(CMD_A::Unlock),
            43690 => Some(CMD_A::Reload),
            52428 => Some(CMD_A::Enable),
            _ => None,
        }
    }
    #[doc = "Unlock write-protected WDT_DIV and WDT_RLD"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == CMD_A::Unlock
    }
    #[doc = "Reload counter"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == CMD_A::Reload
    }
    #[doc = "Enable WDT. If the hardware watchdog has been enabled, ignore this operation."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CMD_A::Enable
    }
}
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
impl R {
    #[doc = "Bits 0:15 - Command register"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD").field("cmd", &self.cmd()).finish()
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
#[doc = "Command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
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
