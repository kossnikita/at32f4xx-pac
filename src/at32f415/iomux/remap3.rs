#[doc = "Register `REMAP3` reader"]
pub type R = crate::R<REMAP3_SPEC>;
#[doc = "Register `REMAP3` writer"]
pub type W = crate::W<REMAP3_SPEC>;
#[doc = "TMR9 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR9_GMUX_A {
    #[doc = "0: CH1/PA2, CH2/PA3"]
    Mux0 = 0,
    #[doc = "2: CH1/PB14, CH2/PB15"]
    Mux1 = 2,
}
impl From<TMR9_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR9_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR9_GMUX_A {
    type Ux = u8;
}
#[doc = "Field `TMR9_GMUX` reader - TMR9 muxing"]
pub type TMR9_GMUX_R = crate::FieldReader<TMR9_GMUX_A>;
impl TMR9_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TMR9_GMUX_A> {
        match self.bits {
            0 => Some(TMR9_GMUX_A::Mux0),
            2 => Some(TMR9_GMUX_A::Mux1),
            _ => None,
        }
    }
    #[doc = "CH1/PA2, CH2/PA3"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR9_GMUX_A::Mux0
    }
    #[doc = "CH1/PB14, CH2/PB15"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == TMR9_GMUX_A::Mux1
    }
}
#[doc = "Field `TMR9_GMUX` writer - TMR9 muxing"]
pub type TMR9_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TMR9_GMUX_A>;
impl<'a, REG> TMR9_GMUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH1/PA2, CH2/PA3"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR9_GMUX_A::Mux0)
    }
    #[doc = "CH1/PB14, CH2/PB15"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR9_GMUX_A::Mux1)
    }
}
#[doc = "TMR10 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR10_GMUX_A {
    #[doc = "0: CH1/PB8"]
    Mux0 = 0,
    #[doc = "2: CH1/PA6"]
    Mux1 = 2,
}
impl From<TMR10_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR10_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR10_GMUX_A {
    type Ux = u8;
}
#[doc = "Field `TMR10_GMUX` reader - TMR10 muxing"]
pub type TMR10_GMUX_R = crate::FieldReader<TMR10_GMUX_A>;
impl TMR10_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TMR10_GMUX_A> {
        match self.bits {
            0 => Some(TMR10_GMUX_A::Mux0),
            2 => Some(TMR10_GMUX_A::Mux1),
            _ => None,
        }
    }
    #[doc = "CH1/PB8"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR10_GMUX_A::Mux0
    }
    #[doc = "CH1/PA6"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == TMR10_GMUX_A::Mux1
    }
}
#[doc = "Field `TMR10_GMUX` writer - TMR10 muxing"]
pub type TMR10_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TMR10_GMUX_A>;
impl<'a, REG> TMR10_GMUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH1/PB8"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR10_GMUX_A::Mux0)
    }
    #[doc = "CH1/PA6"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR10_GMUX_A::Mux1)
    }
}
#[doc = "TMR11 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR11_GMUX_A {
    #[doc = "0: CH1/PB9"]
    Mux0 = 0,
    #[doc = "2: CH1/PA7"]
    Mux1 = 2,
}
impl From<TMR11_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR11_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR11_GMUX_A {
    type Ux = u8;
}
#[doc = "Field `TMR11_GMUX` reader - TMR11 muxing"]
pub type TMR11_GMUX_R = crate::FieldReader<TMR11_GMUX_A>;
impl TMR11_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TMR11_GMUX_A> {
        match self.bits {
            0 => Some(TMR11_GMUX_A::Mux0),
            2 => Some(TMR11_GMUX_A::Mux1),
            _ => None,
        }
    }
    #[doc = "CH1/PB9"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR11_GMUX_A::Mux0
    }
    #[doc = "CH1/PA7"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == TMR11_GMUX_A::Mux1
    }
}
#[doc = "Field `TMR11_GMUX` writer - TMR11 muxing"]
pub type TMR11_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TMR11_GMUX_A>;
impl<'a, REG> TMR11_GMUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH1/PB9"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR11_GMUX_A::Mux0)
    }
    #[doc = "CH1/PA7"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR11_GMUX_A::Mux1)
    }
}
impl R {
    #[doc = "Bits 0:3 - TMR9 muxing"]
    #[inline(always)]
    pub fn tmr9_gmux(&self) -> TMR9_GMUX_R {
        TMR9_GMUX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - TMR10 muxing"]
    #[inline(always)]
    pub fn tmr10_gmux(&self) -> TMR10_GMUX_R {
        TMR10_GMUX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TMR11 muxing"]
    #[inline(always)]
    pub fn tmr11_gmux(&self) -> TMR11_GMUX_R {
        TMR11_GMUX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP3")
            .field("tmr11_gmux", &format_args!("{}", self.tmr11_gmux().bits()))
            .field("tmr10_gmux", &format_args!("{}", self.tmr10_gmux().bits()))
            .field("tmr9_gmux", &format_args!("{}", self.tmr9_gmux().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - TMR9 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9_gmux(&mut self) -> TMR9_GMUX_W<REMAP3_SPEC> {
        TMR9_GMUX_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - TMR10 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10_gmux(&mut self) -> TMR10_GMUX_W<REMAP3_SPEC> {
        TMR10_GMUX_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - TMR11 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11_gmux(&mut self) -> TMR11_GMUX_W<REMAP3_SPEC> {
        TMR11_GMUX_W::new(self, 8)
    }
}
#[doc = "IO MUX remap register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP3_SPEC;
impl crate::RegisterSpec for REMAP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap3::R`](R) reader structure"]
impl crate::Readable for REMAP3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap3::W`](W) writer structure"]
impl crate::Writable for REMAP3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP3 to value 0"]
impl crate::Resettable for REMAP3_SPEC {
    const RESET_VALUE: u32 = 0;
}
