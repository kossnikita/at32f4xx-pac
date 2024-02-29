#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `WIN` reader - Window value"]
pub type WIN_R = crate::FieldReader;
#[doc = "Field `WIN` writer - Window value"]
pub type WIN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Clock division value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIV_A {
    #[doc = "0: PCLK1 divided by 4096"]
    Div4096 = 0,
    #[doc = "1: PCLK1 divided by 8192"]
    Div8192 = 1,
    #[doc = "2: PCLK1 divided by 16384"]
    Div16384 = 2,
    #[doc = "3: PCLK1 divided by 32768"]
    Div32768 = 3,
}
impl From<DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIV_A {
    type Ux = u8;
}
#[doc = "Field `DIV` reader - Clock division value"]
pub type DIV_R = crate::FieldReader<DIV_A>;
impl DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIV_A {
        match self.bits {
            0 => DIV_A::Div4096,
            1 => DIV_A::Div8192,
            2 => DIV_A::Div16384,
            3 => DIV_A::Div32768,
            _ => unreachable!(),
        }
    }
    #[doc = "PCLK1 divided by 4096"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == DIV_A::Div4096
    }
    #[doc = "PCLK1 divided by 8192"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == DIV_A::Div8192
    }
    #[doc = "PCLK1 divided by 16384"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == DIV_A::Div16384
    }
    #[doc = "PCLK1 divided by 32768"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == DIV_A::Div32768
    }
}
#[doc = "Field `DIV` writer - Clock division value"]
pub type DIV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DIV_A>;
impl<'a, REG> DIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCLK1 divided by 4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::Div4096)
    }
    #[doc = "PCLK1 divided by 8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::Div8192)
    }
    #[doc = "PCLK1 divided by 16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::Div16384)
    }
    #[doc = "PCLK1 divided by 32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(DIV_A::Div32768)
    }
}
#[doc = "Reload counter interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rldienr {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Rldienr> for bool {
    #[inline(always)]
    fn from(variant: Rldienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLDIEN` reader - Reload counter interrupt"]
pub type RLDIEN_R = crate::BitReader<Rldienr>;
impl RLDIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rldienr {
        match self.bits {
            false => Rldienr::Disabled,
            true => Rldienr::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rldienr::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rldienr::Enabled
    }
}
#[doc = "Reload counter interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RldienwWO {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<RldienwWO> for bool {
    #[inline(always)]
    fn from(variant: RldienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLDIEN` writer - Reload counter interrupt"]
pub type RLDIEN_W<'a, REG> = crate::BitWriter<'a, REG, RldienwWO>;
impl<'a, REG> RLDIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RldienwWO::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RldienwWO::Enable)
    }
}
impl R {
    #[doc = "Bits 0:6 - Window value"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Clock division value"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Reload counter interrupt"]
    #[inline(always)]
    pub fn rldien(&self) -> RLDIEN_R {
        RLDIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("win", &format_args!("{}", self.win().bits()))
            .field("div", &format_args!("{}", self.div().bits()))
            .field("rldien", &format_args!("{}", self.rldien().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Window value"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WIN_W<CFG_SPEC> {
        WIN_W::new(self, 0)
    }
    #[doc = "Bits 7:8 - Clock division value"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<CFG_SPEC> {
        DIV_W::new(self, 7)
    }
    #[doc = "Bit 9 - Reload counter interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rldien(&mut self) -> RLDIEN_W<CFG_SPEC> {
        RLDIEN_W::new(self, 9)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG to value 0x7f"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
